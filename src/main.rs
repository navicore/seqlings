//! Seqlings - Interactive exercises for learning Seq
//!
//! A rustlings-inspired tool for learning stack-based programming with Seq.

mod exercise;
mod runner;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use colored::Colorize;
use exercise::{Exercise, ExerciseStatus, load_exercises};
use include_dir::{Dir, include_dir};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Path (relative to cwd) where the status cache lives between runs.
const STATE_FILE: &str = ".seqlings-state.json";

// Embed exercise files at compile time
static EXERCISES_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/exercises");
static SOLUTIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/solutions");
static HINTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/hints");

/// Serializable timestamp (whole seconds + sub-second nanos since UNIX_EPOCH).
/// `SystemTime` itself doesn't implement serde, so we round-trip through this.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
struct StoredMtime {
    secs: u64,
    nanos: u32,
}

impl StoredMtime {
    fn from_system(t: SystemTime) -> Self {
        let d = t.duration_since(UNIX_EPOCH).unwrap_or_default();
        Self {
            secs: d.as_secs(),
            nanos: d.subsec_nanos(),
        }
    }
}

/// Identity fingerprint for the `seqc` binary. If any of these change, all
/// cached statuses are invalidated — the user may have upgraded the
/// compiler and previously-passing exercises could now fail.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
struct SeqcIdentity {
    path: String,
    mtime: StoredMtime,
}

impl SeqcIdentity {
    /// Resolve the current `seqc` on PATH and stat it. Returns an empty
    /// identity (path="") if seqc can't be found — that just means every
    /// run will be a cold cache, which is safe.
    fn current() -> Self {
        let path = match resolve_seqc_path() {
            Some(p) => p,
            None => return Self::default(),
        };
        let mtime = std::fs::metadata(&path)
            .and_then(|m| m.modified())
            .map(StoredMtime::from_system)
            .unwrap_or_default();
        Self {
            path: path.to_string_lossy().into_owned(),
            mtime,
        }
    }
}

/// Walk PATH manually to locate `seqc`. Avoids adding a `which` dependency.
fn resolve_seqc_path() -> Option<PathBuf> {
    let path_env = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_env) {
        let candidate = dir.join("seqc");
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct CacheEntry {
    mtime: StoredMtime,
    status: ExerciseStatus,
}

/// On-disk shape of the cache. The header is checked on load — if the
/// compiler identity has changed, the entries are discarded.
#[derive(Default, Serialize, Deserialize)]
struct PersistedCache {
    seqc: SeqcIdentity,
    entries: HashMap<PathBuf, CacheEntry>,
}

/// Cache for exercise status to avoid repeated compiler invocations.
/// Persists to `.seqlings-state.json` between runs and invalidates when
/// the `seqc` binary changes.
struct StatusCache {
    seqc: SeqcIdentity,
    entries: HashMap<PathBuf, CacheEntry>,
    /// Tracks whether any entry has been added or changed since the last
    /// save — used to skip pointless disk writes.
    dirty: bool,
}

impl StatusCache {
    /// Construct an empty cache (no on-disk read).
    fn new() -> Self {
        Self {
            seqc: SeqcIdentity::current(),
            entries: HashMap::new(),
            dirty: false,
        }
    }

    /// Construct a cache from `.seqlings-state.json` if present and
    /// compatible with the current `seqc`. On any error or mismatch,
    /// returns a fresh empty cache.
    fn load_or_new() -> Self {
        let current = SeqcIdentity::current();
        let bytes = match std::fs::read(STATE_FILE) {
            Ok(b) => b,
            Err(_) => return Self::new(),
        };
        let persisted: PersistedCache = match serde_json::from_slice(&bytes) {
            Ok(p) => p,
            Err(_) => return Self::new(),
        };
        if persisted.seqc != current {
            // seqc changed (likely an upgrade) — discard everything so
            // every exercise is re-verified against the new compiler.
            return Self {
                seqc: current,
                entries: HashMap::new(),
                dirty: false,
            };
        }
        Self {
            seqc: current,
            entries: persisted.entries,
            dirty: false,
        }
    }

    /// Write the cache to `.seqlings-state.json` if it has changed since
    /// the last save. Best-effort: errors are silently ignored so a
    /// read-only working directory doesn't break the watch loop.
    fn save(&mut self) {
        if !self.dirty {
            return;
        }
        let persisted = PersistedCache {
            seqc: self.seqc.clone(),
            entries: self.entries.clone(),
        };
        if let Ok(bytes) = serde_json::to_vec_pretty(&persisted)
            && std::fs::write(STATE_FILE, bytes).is_ok()
        {
            self.dirty = false;
        }
    }

    /// Get the status of an exercise, using cache when possible.
    /// This is the main optimization: we only re-run the compiler
    /// when the file has actually changed.
    fn get_status(&mut self, exercise: &Exercise) -> ExerciseStatus {
        // Get current file mtime
        let current_mtime = match std::fs::metadata(&exercise.path) {
            Ok(meta) => meta.modified().ok().map(StoredMtime::from_system),
            Err(_) => return ExerciseStatus::CompileError,
        };

        // Quick pre-filter: if file contains "# I AM NOT DONE", skip expensive checks
        // This is a cheap read that can short-circuit compiler invocation
        if let Ok(content) = std::fs::read_to_string(&exercise.path)
            && content.contains("# I AM NOT DONE")
        {
            // Update cache with NotDone status
            if let Some(mtime) = current_mtime {
                self.insert(
                    exercise.path.clone(),
                    CacheEntry {
                        mtime,
                        status: ExerciseStatus::NotDone,
                    },
                );
            }
            return ExerciseStatus::NotDone;
        }

        // Check cache: if mtime unchanged, return cached status
        if let Some(mtime) = current_mtime
            && let Some(entry) = self.entries.get(&exercise.path)
            && entry.mtime == mtime
        {
            return entry.status.clone();
        }

        // Cache miss or file changed - run the full status check
        let status = exercise.status();

        // Update cache
        if let Some(mtime) = current_mtime {
            self.insert(
                exercise.path.clone(),
                CacheEntry {
                    mtime,
                    status: status.clone(),
                },
            );
        }

        status
    }

    fn insert(&mut self, path: PathBuf, entry: CacheEntry) {
        let unchanged = self
            .entries
            .get(&path)
            .is_some_and(|existing| existing.mtime == entry.mtime && existing.status == entry.status);
        if !unchanged {
            self.dirty = true;
        }
        self.entries.insert(path, entry);
    }

    /// Clear the cache (useful for commands that need fresh data)
    #[allow(dead_code)]
    fn clear(&mut self) {
        if !self.entries.is_empty() {
            self.dirty = true;
        }
        self.entries.clear();
    }
}

#[derive(Parser)]
#[command(name = "seqlings")]
#[command(version, about = "Interactive exercises for learning Seq")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new seqlings project directory
    Init {
        /// Directory name (defaults to "my-seqlings")
        #[arg(default_value = "my-seqlings")]
        path: PathBuf,
    },
    /// Watch for file changes and auto-verify exercises
    Watch {
        /// Filter to a specific chapter (e.g., "07" or "07-conditionals")
        #[arg(short, long)]
        chapter: Option<String>,
    },
    /// List all exercises with their status
    List {
        /// Filter to a specific chapter (e.g., "07" or "07-conditionals")
        #[arg(short, long)]
        chapter: Option<String>,
    },
    /// Show hint for the current or specified exercise
    Hint {
        /// Exercise name (optional, defaults to current)
        name: Option<String>,
    },
    /// Reset an exercise to its original state
    Reset {
        /// Exercise name (optional, defaults to current)
        name: Option<String>,
    },
    /// Verify all exercises and show progress
    Verify,
    /// Skip to the next exercise
    Next,
    /// Refresh exercises from the embedded corpus, preserving in-progress work
    ///
    /// Replaces only exercises whose source file still has the
    /// "# I AM NOT DONE" marker (the "not started" signal). Files
    /// you've touched are left alone. Hints and solutions are
    /// always refreshed. Use --force to override per file.
    Update {
        /// Show what would happen without writing anything
        #[arg(long)]
        dry_run: bool,
        /// Force-replace a specific exercise even if you've touched it.
        /// Use the path under exercises/, e.g. "09-recursion/05-mutual"
        /// (with or without the .seq extension). Repeatable.
        #[arg(long, value_name = "EXERCISE")]
        force: Vec<String>,
    },
    /// Print a shell completion script to stdout
    ///
    /// Example: seqlings completions zsh > ~/.zfunc/_seqlings
    Completions {
        /// Target shell (bash, zsh, fish, powershell, elvish)
        shell: Shell,
    },
}

fn main() {
    let cli = Cli::parse();

    // Handle commands that don't need an exercise tree on disk.
    match cli.command {
        Some(Commands::Init { ref path }) => {
            cmd_init(path);
            return;
        }
        Some(Commands::Completions { shell }) => {
            cmd_completions(shell);
            return;
        }
        _ => {}
    }

    // Load exercises
    let exercises = match load_exercises() {
        Ok(ex) => ex,
        Err(e) => {
            eprintln!("{} {}", "Error loading exercises:".red(), e);
            eprintln!(
                "\n{}",
                "Hint: Run 'seqlings init' to create a new project.".yellow()
            );
            process::exit(1);
        }
    };

    if exercises.is_empty() {
        eprintln!("{}", "No exercises found in exercises/info.toml".red());
        eprintln!(
            "\n{}",
            "Hint: Run 'seqlings init' to create a new project.".yellow()
        );
        process::exit(1);
    }

    match cli.command {
        Some(Commands::Init { .. }) | Some(Commands::Completions { .. }) => unreachable!(), // Handled above
        Some(Commands::Watch { chapter }) => {
            let filtered = filter_by_chapter(&exercises, chapter.as_deref());
            cmd_watch(&filtered);
        }
        Some(Commands::List { chapter }) => {
            let filtered = filter_by_chapter(&exercises, chapter.as_deref());
            cmd_list(&filtered);
        }
        Some(Commands::Hint { name }) => cmd_hint(&exercises, name),
        Some(Commands::Reset { name }) => cmd_reset(&exercises, name),
        Some(Commands::Verify) => cmd_verify(&exercises),
        Some(Commands::Next) => cmd_next(&exercises),
        Some(Commands::Update { dry_run, force }) => cmd_update(dry_run, &force),
        None => cmd_watch(&exercises), // Default to watch mode
    }
}

/// Filter exercises to a specific chapter by prefix match
fn filter_by_chapter(exercises: &[Exercise], chapter: Option<&str>) -> Vec<Exercise> {
    match chapter {
        None => exercises.to_vec(),
        Some(prefix) => {
            let filtered: Vec<Exercise> = exercises
                .iter()
                .filter(|e| {
                    // Extract chapter directory name from path (e.g., "07-conditionals")
                    let chapter_name = e
                        .path
                        .parent()
                        .and_then(|p| p.file_name())
                        .and_then(|s| s.to_str())
                        .unwrap_or("");
                    // Match if chapter name starts with the prefix
                    chapter_name.starts_with(prefix)
                })
                .cloned()
                .collect();

            if filtered.is_empty() {
                eprintln!(
                    "{} No exercises found for chapter '{}'",
                    "Warning:".yellow(),
                    prefix
                );
                eprintln!("Available chapters:");
                // Show unique chapter names
                let mut chapters: Vec<&str> = exercises
                    .iter()
                    .filter_map(|e| {
                        e.path
                            .parent()
                            .and_then(|p| p.file_name())
                            .and_then(|s| s.to_str())
                    })
                    .collect();
                chapters.sort();
                chapters.dedup();
                for ch in chapters {
                    eprintln!("  {}", ch);
                }
                process::exit(1);
            }

            println!(
                "{} Filtering to chapter '{}' ({} exercises)\n",
                "Note:".cyan(),
                prefix,
                filtered.len()
            );
            filtered
        }
    }
}

/// Print a shell completion script to stdout.
fn cmd_completions(shell: Shell) {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "seqlings", &mut std::io::stdout());
}

/// Per-file decision the update command makes for an exercise.
enum UpdateAction {
    /// On-disk file does not exist — extract it.
    Create,
    /// On-disk content matches embedded — nothing to do.
    AlreadyCurrent,
    /// User hasn't started (marker still present); replace freely.
    Replace,
    /// User passed --force for this file; replace despite touched state.
    ForceReplace,
    /// User has touched this file (no marker); leave alone.
    Preserve,
}

/// Recursively collect every file in an embedded directory.
fn collect_embedded_files<'a>(dir: &'a Dir<'a>, out: &mut Vec<&'a include_dir::File<'a>>) {
    for entry in dir.entries() {
        match entry {
            include_dir::DirEntry::File(f) => out.push(f),
            include_dir::DirEntry::Dir(d) => collect_embedded_files(d, out),
        }
    }
}

/// Decide what to do with one exercise file.
fn classify_exercise(on_disk: &Path, embedded: &[u8], forced: bool) -> UpdateAction {
    let on_disk_bytes = match std::fs::read(on_disk) {
        Ok(b) => b,
        Err(_) => return UpdateAction::Create,
    };
    if on_disk_bytes == embedded {
        return UpdateAction::AlreadyCurrent;
    }
    if forced {
        return UpdateAction::ForceReplace;
    }
    let text = String::from_utf8_lossy(&on_disk_bytes);
    if text.contains("# I AM NOT DONE") {
        UpdateAction::Replace
    } else {
        UpdateAction::Preserve
    }
}

/// Refresh an entire embedded tree to disk, overwriting any existing files.
/// Used for hints/ and solutions/ which are reference material, not user files.
fn refresh_tree(dir: &Dir<'_>, target_root: &str) -> std::io::Result<()> {
    let mut files = Vec::new();
    collect_embedded_files(dir, &mut files);
    for f in files {
        let target = PathBuf::from(target_root).join(f.path());
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&target, f.contents())?;
    }
    Ok(())
}

/// Refresh untouched exercises from the embedded corpus, leaving in-progress
/// or completed work alone. Hints and solutions always refresh.
fn cmd_update(dry_run: bool, force: &[String]) {
    // Normalize --force values: strip leading `exercises/`, append `.seq`
    // if missing. Lets the user write either form.
    let force_set: std::collections::HashSet<String> = force
        .iter()
        .map(|s| {
            let stripped = s
                .trim_start_matches("./")
                .trim_start_matches("exercises/")
                .to_string();
            if stripped.ends_with(".seq") {
                stripped
            } else {
                format!("{stripped}.seq")
            }
        })
        .collect();

    let mut files = Vec::new();
    collect_embedded_files(&EXERCISES_DIR, &mut files);
    // Skip info.toml — that's the manifest, not an exercise. Replacing
    // it is desirable (new exercises get registered) and we treat it
    // separately below to avoid noise in the per-exercise summary.
    let exercise_files: Vec<&include_dir::File<'_>> = files
        .into_iter()
        .filter(|f| f.path() != Path::new("info.toml"))
        .collect();

    let mut created: Vec<String> = Vec::new();
    let mut replaced: Vec<String> = Vec::new();
    let mut force_replaced: Vec<String> = Vec::new();
    let mut preserved: Vec<String> = Vec::new();
    let mut current_count = 0usize;
    let mut errors: Vec<String> = Vec::new();

    for f in &exercise_files {
        let rel = f.path();
        let rel_str = rel.to_string_lossy().to_string();
        let on_disk = PathBuf::from("exercises").join(rel);
        let forced = force_set.contains(&rel_str);

        match classify_exercise(&on_disk, f.contents(), forced) {
            UpdateAction::AlreadyCurrent => current_count += 1,
            UpdateAction::Create => {
                created.push(rel_str);
                if !dry_run && let Err(e) = write_file(&on_disk, f.contents()) {
                    errors.push(format!("create {}: {e}", on_disk.display()));
                }
            }
            UpdateAction::Replace => {
                replaced.push(rel_str);
                if !dry_run && let Err(e) = write_file(&on_disk, f.contents()) {
                    errors.push(format!("replace {}: {e}", on_disk.display()));
                }
            }
            UpdateAction::ForceReplace => {
                force_replaced.push(rel_str);
                if !dry_run && let Err(e) = write_file(&on_disk, f.contents()) {
                    errors.push(format!("force-replace {}: {e}", on_disk.display()));
                }
            }
            UpdateAction::Preserve => preserved.push(rel_str),
        }
    }

    // info.toml and reference trees (solutions/, hints/) refresh wholesale.
    if !dry_run {
        if let Some(info) = EXERCISES_DIR.get_file("info.toml")
            && let Err(e) = write_file(Path::new("exercises/info.toml"), info.contents())
        {
            errors.push(format!("refresh exercises/info.toml: {e}"));
        }
        if let Err(e) = refresh_tree(&SOLUTIONS_DIR, "solutions") {
            errors.push(format!("refresh solutions/: {e}"));
        }
        if let Err(e) = refresh_tree(&HINTS_DIR, "hints") {
            errors.push(format!("refresh hints/: {e}"));
        }
    }

    print_update_summary(
        &created,
        &replaced,
        &force_replaced,
        &preserved,
        current_count,
        &errors,
        dry_run,
    );

    if !errors.is_empty() {
        process::exit(1);
    }
}

fn write_file(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, bytes)
}

#[allow(clippy::too_many_arguments)]
fn print_update_summary(
    created: &[String],
    replaced: &[String],
    force_replaced: &[String],
    preserved: &[String],
    current_count: usize,
    errors: &[String],
    dry_run: bool,
) {
    if dry_run {
        println!("\n{}", "[dry run] no files were modified".yellow().bold());
    }

    if !created.is_empty() {
        println!(
            "\n{}",
            format!("Created ({}):", created.len()).green().bold()
        );
        for n in created {
            println!("  {} {}", "+".green(), n);
        }
    }

    if !replaced.is_empty() {
        println!(
            "\n{}",
            format!("Replaced ({}):", replaced.len()).cyan().bold()
        );
        for n in replaced {
            println!("  {} {}", "~".cyan(), n);
        }
    }

    if !force_replaced.is_empty() {
        println!(
            "\n{}",
            format!("Force-replaced ({}):", force_replaced.len())
                .yellow()
                .bold()
        );
        for n in force_replaced {
            println!("  {} {}", "!".yellow(), n);
        }
    }

    if !preserved.is_empty() {
        println!(
            "\n{}",
            format!(
                "Preserved ({}, your work was kept; pass --force <path> to override):",
                preserved.len()
            )
            .bold()
        );
        for n in preserved {
            println!("  {} {}", "-".dimmed(), n.dimmed());
        }
    }

    println!(
        "\n{} already up to date · {} created · {} replaced · {} force-replaced · {} preserved",
        current_count,
        created.len(),
        replaced.len(),
        force_replaced.len(),
        preserved.len()
    );

    if !errors.is_empty() {
        println!("\n{} {}", "Errors:".red().bold(), errors.len());
        for e in errors {
            println!("  {} {}", "x".red(), e);
        }
    }
}

/// Initialize a new seqlings project directory
fn cmd_init(path: &Path) {
    // Check if directory already exists
    if path.exists() {
        eprintln!(
            "{} Directory '{}' already exists.",
            "Error:".red(),
            path.display()
        );
        eprintln!("Choose a different name or remove the existing directory.");
        process::exit(1);
    }

    println!(
        "{} Initializing seqlings project in '{}'...",
        "→".cyan(),
        path.display()
    );

    // Create the main directory
    if let Err(e) = std::fs::create_dir_all(path) {
        eprintln!("{} Failed to create directory: {}", "Error:".red(), e);
        process::exit(1);
    }

    // Extract exercises
    let exercises_path = path.join("exercises");
    if let Err(e) = extract_dir(&EXERCISES_DIR, &exercises_path) {
        eprintln!("{} Failed to extract exercises: {}", "Error:".red(), e);
        process::exit(1);
    }
    println!("  {} exercises/", "✓".green());

    // Extract solutions
    let solutions_path = path.join("solutions");
    if let Err(e) = extract_dir(&SOLUTIONS_DIR, &solutions_path) {
        eprintln!("{} Failed to extract solutions: {}", "Error:".red(), e);
        process::exit(1);
    }
    println!("  {} solutions/", "✓".green());

    // Extract hints
    let hints_path = path.join("hints");
    if let Err(e) = extract_dir(&HINTS_DIR, &hints_path) {
        eprintln!("{} Failed to extract hints: {}", "Error:".red(), e);
        process::exit(1);
    }
    println!("  {} hints/", "✓".green());

    println!("\n{} Project initialized successfully!", "✓".green().bold());
    println!("\nTo get started:");
    println!("  {} {}", "cd".cyan(), path.display());
    println!("  {}", "seqlings".cyan());
}

/// Extract an embedded directory to the filesystem
fn extract_dir(dir: &Dir, target: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(target)?;

    for entry in dir.entries() {
        let entry_path = target.join(entry.path().file_name().unwrap_or_default());

        match entry {
            include_dir::DirEntry::Dir(subdir) => {
                extract_dir(subdir, &entry_path)?;
            }
            include_dir::DirEntry::File(file) => {
                if let Some(parent) = entry_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&entry_path, file.contents())?;
            }
        }
    }

    Ok(())
}

const BANNER: &str = r#"
                  _ _
     ___  ___  __| | (_)_ __   __ _ ___
    / __|/ _ \/ _` | | | '_ \ / _` / __|
    \__ \  __/ (_| | | | | | | (_| \__ \
    |___/\___|\__, |_|_|_| |_|\__, |___/
                 |_|          |___/
"#;

fn print_banner() {
    println!("{}", BANNER.green().bold());
    println!("    {}\n", "\"Look out! Broken programs below!\"".dimmed());
}

/// Watch mode: continuously monitor exercises and provide feedback
fn cmd_watch(exercises: &[Exercise]) {
    // Restore status cache from disk to avoid re-checking exercises whose
    // files (and the compiler) haven't changed since the last run.
    let mut cache = StatusCache::load_or_new();

    // Warm up cache silently (transient progress indicator gets cleared before first frame)
    use std::io::Write;
    print!("{}", "Checking exercises...".dimmed());
    std::io::stdout().flush().ok();
    for ex in exercises.iter() {
        cache.get_status(ex);
    }
    cache.save();

    // First frame: clear away the warmup line, render banner, then assessment
    clear_screen();
    print_banner();
    let mut current_exercise_name = String::new();
    display_current_exercise(exercises, &mut current_exercise_name, &mut cache);

    loop {
        std::thread::sleep(Duration::from_millis(250));

        // Check files every 250ms
        let mut changed = false;
        for ex in exercises {
            if let Ok(meta) = std::fs::metadata(&ex.path)
                && let Ok(mtime) = meta.modified()
                && mtime.elapsed().unwrap_or(Duration::from_secs(1000)) < Duration::from_millis(500)
            {
                changed = true;
                break;
            }
        }

        if changed {
            clear_screen();
            print_banner();
            display_current_exercise(exercises, &mut current_exercise_name, &mut cache);
            cache.save();
        }
    }
}

fn clear_screen() {
    // ANSI escape to clear screen and move cursor to top
    print!("\x1B[2J\x1B[1;1H");
    use std::io::Write;
    std::io::stdout().flush().ok();
}

fn display_current_exercise(
    exercises: &[Exercise],
    previous_name: &mut String,
    cache: &mut StatusCache,
) {
    // Find first incomplete exercise using cached status
    let current = exercises.iter().find(|e| {
        matches!(
            cache.get_status(e),
            ExerciseStatus::NotDone | ExerciseStatus::CompileError | ExerciseStatus::TestFail
        )
    });

    match current {
        Some(exercise) => {
            let status = cache.get_status(exercise);

            // Check if we moved to a new exercise
            if !previous_name.is_empty() && *previous_name != exercise.name {
                println!(
                    "{} Completed {}!\n",
                    "✓".green().bold(),
                    previous_name.cyan()
                );
            }
            *previous_name = exercise.name.clone();

            // Show exercise header
            println!(
                "{} {}\n",
                "Current exercise:".green().bold(),
                exercise.name.cyan()
            );

            // Show file path (relative to the project root)
            println!("  File: {}", exercise.path.display().to_string().dimmed());

            // Show status with details
            match status {
                ExerciseStatus::NotDone => {
                    println!("  Status: {}\n", "Waiting for you to start...".yellow());

                    // Show exercise description
                    if let Ok(content) = std::fs::read_to_string(&exercise.path) {
                        let header: Vec<&str> = content
                            .lines()
                            .take_while(|l| l.starts_with('#'))
                            .filter(|l| !l.contains("I AM NOT DONE"))
                            .collect();
                        for line in header {
                            println!("  {}", line.dimmed());
                        }
                    }

                    println!();
                    println!(
                        "  {}",
                        "Delete the '# I AM NOT DONE' line when you've solved it.".yellow()
                    );
                }
                ExerciseStatus::CompileError => {
                    println!("  Status: {}\n", "Compile Error".red().bold());

                    if let Err(e) = runner::compile(&exercise.path) {
                        // Show first few lines of error
                        for line in e.lines().take(15) {
                            println!("  {}", line.red());
                        }
                    }
                }
                ExerciseStatus::TestFail => {
                    println!("  Status: {}\n", "Tests Failed".red().bold());

                    match runner::run_tests(&exercise.path) {
                        Ok(output) | Err(output) => {
                            // seqc prints `test-X ... FAILED` twice — once
                            // in the per-file summary and again in the
                            // TEST FAILURES: section where `at line N:
                            // expected X, got Y` detail is attached. We
                            // prefer the detail section; fall back to the
                            // full output if the marker is absent.
                            let section = match output.find("TEST FAILURES:") {
                                Some(i) => &output[i + "TEST FAILURES:".len()..],
                                None => output.as_str(),
                            };
                            let mut shown = 0;
                            for line in section.lines() {
                                if shown >= 20 {
                                    break;
                                }
                                let trimmed = line.trim_end();
                                if trimmed.is_empty() {
                                    continue;
                                }
                                // Drop the tempfile path header that
                                // introduces each failed test — we show
                                // the real exercise path under `File:`.
                                if trimmed.starts_with('/') && trimmed.contains("::") {
                                    continue;
                                }
                                if line.contains("at line") {
                                    println!("  {}", line.yellow());
                                } else if line.contains("FAIL") || line.contains("panicked") {
                                    println!("  {}", line.red());
                                } else if line.contains(" ok") {
                                    println!("  {}", line.green());
                                } else {
                                    println!("  {}", line);
                                }
                                shown += 1;
                            }
                        }
                    }
                }
                ExerciseStatus::Done => {
                    // Shouldn't happen in this branch, but handle it
                    println!("  Status: {}", "Done".green());
                }
            }

            println!();
            println!("  {} seqlings hint", "Hint:".cyan());
            show_progress(exercises, cache);
        }
        None => {
            // All done!
            clear_screen();
            println!("\n{}", "=".repeat(50).green());
            println!(
                "{}",
                "  Congratulations! You've completed all exercises!"
                    .green()
                    .bold()
            );
            println!("{}\n", "=".repeat(50).green());
            show_progress(exercises, cache);
            println!("\n{}", "You're now a Seq programmer!".cyan().bold());
            process::exit(0);
        }
    }
}

/// Open exercise in editor (alternative to watch mode)
#[allow(dead_code)]
fn cmd_run(exercises: &[Exercise]) {
    let mut cache = StatusCache::load_or_new();

    // Find first incomplete exercise
    let current = exercises.iter().find(|e| {
        matches!(
            cache.get_status(e),
            ExerciseStatus::NotDone | ExerciseStatus::CompileError | ExerciseStatus::TestFail
        )
    });

    match current {
        Some(exercise) => {
            let status = cache.get_status(exercise);
            println!(
                "\n{} {}\n",
                "Current exercise:".green().bold(),
                exercise.name.cyan()
            );
            println!("  Path: {}", exercise.path.display());
            println!("  Status: {}", format_status(&status));
            println!();

            // Show the exercise description
            if let Ok(content) = std::fs::read_to_string(&exercise.path) {
                // Extract comment header
                let header: Vec<&str> =
                    content.lines().take_while(|l| l.starts_with('#')).collect();
                for line in header {
                    println!("  {}", line.dimmed());
                }
            }

            println!();
            println!(
                "{}",
                "Open this file in your editor to complete the exercise.".yellow()
            );
            println!("Run {} to see a hint.", "seqlings hint".cyan());
            println!();

            // Open in $EDITOR if set
            if let Ok(editor) = std::env::var("EDITOR") {
                println!("Opening in {}...", editor.cyan());
                let cmd_status = process::Command::new(&editor).arg(&exercise.path).status();

                match cmd_status {
                    Ok(s) if s.success() => {
                        // After editor closes, verify the exercise
                        println!();
                        verify_exercise(exercise);
                    }
                    Ok(_) => {
                        eprintln!("{}", "Editor exited with error".red());
                    }
                    Err(e) => {
                        eprintln!("{} {}", "Failed to open editor:".red(), e);
                    }
                }
            } else {
                println!(
                    "{}",
                    "Set $EDITOR environment variable to open exercises automatically.".dimmed()
                );
            }
        }
        None => {
            println!(
                "\n{}",
                "Congratulations! You've completed all exercises!"
                    .green()
                    .bold()
            );
            show_progress(exercises, &mut cache);
        }
    }
}

/// List all exercises
fn cmd_list(exercises: &[Exercise]) {
    let mut cache = StatusCache::load_or_new();

    println!("\n{}\n", "Seqlings Exercises".green().bold());

    let mut current_topic = String::new();
    for exercise in exercises {
        // Extract topic from path
        let topic = exercise
            .path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        if topic != current_topic {
            println!("\n  {}", topic.cyan().bold());
            current_topic = topic.to_string();
        }

        let status = cache.get_status(exercise);
        let status_icon = match status {
            ExerciseStatus::Done => "✓".green(),
            ExerciseStatus::NotDone => "○".yellow(),
            ExerciseStatus::CompileError => "✗".red(),
            ExerciseStatus::TestFail => "✗".red(),
        };

        println!("    {} {}", status_icon, exercise.name);
    }

    println!();
    show_progress(exercises, &mut cache);
    cache.save();
}

/// Show hint for an exercise
fn cmd_hint(exercises: &[Exercise], name: Option<String>) {
    let mut cache = StatusCache::load_or_new();
    let name_provided = name.is_some();
    let exercise = match &name {
        Some(n) => exercises.iter().find(|e| &e.name == n),
        None => exercises.iter().find(|e| {
            matches!(
                cache.get_status(e),
                ExerciseStatus::NotDone | ExerciseStatus::CompileError | ExerciseStatus::TestFail
            )
        }),
    };

    match exercise {
        Some(ex) => {
            // Construct hint path
            let hint_path = ex.hint_path();
            if hint_path.exists() {
                match std::fs::read_to_string(&hint_path) {
                    Ok(content) => {
                        println!("\n{} {}\n", "Hint for".green(), ex.name.cyan());
                        println!("{}", content);
                    }
                    Err(e) => {
                        eprintln!("{} {}", "Error reading hint:".red(), e);
                    }
                }
            } else {
                println!("\n{} {}", "No hint available for".yellow(), ex.name.cyan());
                println!("Hint file not found: {}", hint_path.display());
            }
        }
        None => {
            if name_provided {
                eprintln!("{}", "Exercise not found".red());
            } else {
                println!("{}", "All exercises complete! No hints needed.".green());
            }
        }
    }
}

/// Reset an exercise to its original stub by restoring the content embedded
/// in the binary at compile time.
fn cmd_reset(exercises: &[Exercise], name: Option<String>) {
    let mut cache = StatusCache::load_or_new();
    let exercise = match name {
        Some(n) => exercises.iter().find(|e| e.name == n),
        None => exercises.iter().find(|e| {
            matches!(
                cache.get_status(e),
                ExerciseStatus::NotDone | ExerciseStatus::CompileError | ExerciseStatus::TestFail
            )
        }),
    };

    match exercise {
        Some(ex) => {
            // EXERCISES_DIR is rooted at the exercises/ directory, so strip
            // the leading "exercises" segment from ex.path to get the key.
            let relative = match ex.path.strip_prefix("exercises") {
                Ok(r) => r,
                Err(_) => {
                    eprintln!(
                        "{} {}",
                        "Cannot resolve exercise path:".red(),
                        ex.path.display()
                    );
                    return;
                }
            };
            match EXERCISES_DIR.get_file(relative) {
                Some(embedded) => match std::fs::write(&ex.path, embedded.contents()) {
                    Ok(()) => println!("{} {}", "Reset".green(), ex.name.cyan()),
                    Err(e) => eprintln!("{} {}", "Error writing exercise:".red(), e),
                },
                None => {
                    eprintln!(
                        "{} Original for '{}' not found in embedded corpus.",
                        "Error:".red(),
                        ex.name
                    );
                }
            }
        }
        None => {
            eprintln!("{}", "Exercise not found".red());
        }
    }
}

/// Verify all exercises
fn cmd_verify(exercises: &[Exercise]) {
    let mut cache = StatusCache::load_or_new();

    println!("\n{}\n", "Verifying all exercises...".green().bold());

    for exercise in exercises {
        let status = cache.get_status(exercise);
        let status_str = format_status(&status);
        let icon = match status {
            ExerciseStatus::Done => "✓".green(),
            _ => "✗".red(),
        };
        println!("  {} {} - {}", icon, exercise.name, status_str);
    }

    println!();
    show_progress(exercises, &mut cache);
    cache.save();
}

/// Skip to next exercise
fn cmd_next(exercises: &[Exercise]) {
    let mut cache = StatusCache::load_or_new();

    // Find current incomplete
    let current_idx = exercises.iter().position(|e| {
        matches!(
            cache.get_status(e),
            ExerciseStatus::NotDone | ExerciseStatus::CompileError | ExerciseStatus::TestFail
        )
    });

    match current_idx {
        Some(idx) if idx + 1 < exercises.len() => {
            let next = &exercises[idx + 1];
            println!("Skipping to: {}", next.name.cyan());
            // Mark current as done by removing # NOT DONE
            // (This is a skip, not a completion)
        }
        _ => {
            println!("{}", "No more exercises to skip to.".yellow());
        }
    }
    cache.save();
}

/// Verify a single exercise and show result
#[allow(dead_code)]
fn verify_exercise(exercise: &Exercise) {
    let mut cache = StatusCache::load_or_new();
    let status = cache.get_status(exercise);
    println!("{} {}", "Exercise status:".bold(), format_status(&status));

    match status {
        ExerciseStatus::Done => {
            println!("{}", "Great job! Run 'seqlings' to continue.".green());
        }
        ExerciseStatus::CompileError => {
            // Try to compile and show error
            if let Err(e) = runner::compile(&exercise.path) {
                println!("\n{}\n{}", "Compile error:".red(), e);
            }
        }
        ExerciseStatus::TestFail => {
            // Try to run and show failure
            match runner::run_tests(&exercise.path) {
                Ok(output) => println!("\n{}\n{}", "Test output:".yellow(), output),
                Err(e) => println!("\n{}\n{}", "Test error:".red(), e),
            }
        }
        ExerciseStatus::NotDone => {
            println!(
                "{}",
                "Remove '# NOT DONE' when you've completed the exercise.".yellow()
            );
        }
    }
}

fn format_status(status: &ExerciseStatus) -> colored::ColoredString {
    match status {
        ExerciseStatus::Done => "Done".green(),
        ExerciseStatus::NotDone => "Not Done".yellow(),
        ExerciseStatus::CompileError => "Compile Error".red(),
        ExerciseStatus::TestFail => "Test Failed".red(),
    }
}

fn show_progress(exercises: &[Exercise], cache: &mut StatusCache) {
    let done = exercises
        .iter()
        .filter(|e| matches!(cache.get_status(e), ExerciseStatus::Done))
        .count();
    let total = exercises.len();
    let pct = (done as f64 / total as f64 * 100.0) as usize;

    // Eighth-block characters give sub-cell precision so the bar
    // moves smoothly even on small absolute changes.
    let bar_width = 30;
    let total_eighths = bar_width * 8;
    let filled_eighths = (done * total_eighths) / total;
    let full_cells = filled_eighths / 8;
    let partial_eighths = filled_eighths % 8;

    // Index = number of eighths filled in the partial cell.
    const PARTIAL: [&str; 8] = ["", "▏", "▎", "▍", "▌", "▋", "▊", "▉"];

    let full = "█".repeat(full_cells);
    let partial = PARTIAL[partial_eighths];
    let used_cells = full_cells + usize::from(partial_eighths > 0);
    let empty = "░".repeat(bar_width - used_cells);

    println!(
        "\nProgress: [{}{}{}] {}/{} ({}%)",
        full.green(),
        partial.green(),
        empty.dimmed(),
        done,
        total,
        pct
    );
}
