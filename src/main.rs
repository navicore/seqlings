//! Seqlings - Interactive exercises for learning Seq
//!
//! A rustlings-inspired tool for learning stack-based programming with Seq.

mod exercise;
mod runner;

use clap::{Parser, Subcommand};
use colored::Colorize;
use exercise::{Exercise, ExerciseStatus, load_exercises};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process;
use std::time::{Duration, SystemTime};

/// Cache for exercise status to avoid repeated compiler invocations
struct StatusCache {
    /// Maps exercise path to (last_mtime, cached_status)
    cache: HashMap<PathBuf, (SystemTime, ExerciseStatus)>,
}

impl StatusCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Get the status of an exercise, using cache when possible.
    /// This is the main optimization: we only re-run the compiler
    /// when the file has actually changed.
    fn get_status(&mut self, exercise: &Exercise) -> ExerciseStatus {
        // Get current file mtime
        let current_mtime = match std::fs::metadata(&exercise.path) {
            Ok(meta) => meta.modified().ok(),
            Err(_) => return ExerciseStatus::CompileError,
        };

        // Quick pre-filter: if file contains "# I AM NOT DONE", skip expensive checks
        // This is a cheap read that can short-circuit compiler invocation
        if let Ok(content) = std::fs::read_to_string(&exercise.path) {
            if content.contains("# I AM NOT DONE") {
                // Update cache with NotDone status
                if let Some(mtime) = current_mtime {
                    self.cache.insert(exercise.path.clone(), (mtime, ExerciseStatus::NotDone));
                }
                return ExerciseStatus::NotDone;
            }
        }

        // Check cache: if mtime unchanged, return cached status
        if let Some(mtime) = current_mtime {
            if let Some((cached_mtime, cached_status)) = self.cache.get(&exercise.path) {
                if *cached_mtime == mtime {
                    return cached_status.clone();
                }
            }
        }

        // Cache miss or file changed - run the full status check
        let status = exercise.status();

        // Update cache
        if let Some(mtime) = current_mtime {
            self.cache.insert(exercise.path.clone(), (mtime, status.clone()));
        }

        status
    }

    /// Clear the cache (useful for commands that need fresh data)
    #[allow(dead_code)]
    fn clear(&mut self) {
        self.cache.clear();
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
}

fn main() {
    let cli = Cli::parse();

    // Load exercises
    let exercises = match load_exercises() {
        Ok(ex) => ex,
        Err(e) => {
            eprintln!("{} {}", "Error loading exercises:".red(), e);
            process::exit(1);
        }
    };

    if exercises.is_empty() {
        eprintln!("{}", "No exercises found in exercises/info.toml".red());
        process::exit(1);
    }

    match cli.command {
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
                    let chapter_name = e.path
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

/// Watch mode: continuously monitor exercises and provide feedback
fn cmd_watch(exercises: &[Exercise]) {
    println!(
        "\n{}",
        "Welcome to seqlings watch mode!".green().bold()
    );
    println!("{}", "Edit exercises in your editor. Progress updates automatically.".dimmed());
    println!("{}", "Press Ctrl+C to exit.\n".dimmed());

    // Create status cache to avoid repeated compiler invocations
    let mut cache = StatusCache::new();

    // Warm up cache with progress indicator
    print!("{}", "Checking exercises...".dimmed());
    use std::io::Write;
    std::io::stdout().flush().ok();

    for (i, ex) in exercises.iter().enumerate() {
        cache.get_status(ex);
        // Show progress dot every 5 exercises
        if (i + 1) % 5 == 0 {
            print!(".");
            std::io::stdout().flush().ok();
        }
    }
    println!(" {}", "done".green());

    // Initial display
    let mut current_exercise_name = String::new();
    display_current_exercise(exercises, &mut current_exercise_name, &mut cache);

    loop {
        std::thread::sleep(Duration::from_millis(250));

        // Check files every 250ms
        let mut changed = false;
        for ex in exercises {
            if let Ok(meta) = std::fs::metadata(&ex.path) {
                if let Ok(mtime) = meta.modified() {
                    if mtime.elapsed().unwrap_or(Duration::from_secs(1000)) < Duration::from_millis(500) {
                        changed = true;
                        break;
                    }
                }
            }
        }

        if changed {
            clear_screen();
            display_current_exercise(exercises, &mut current_exercise_name, &mut cache);
        }
    }
}

fn clear_screen() {
    // ANSI escape to clear screen and move cursor to top
    print!("\x1B[2J\x1B[1;1H");
    use std::io::Write;
    std::io::stdout().flush().ok();
}

fn display_current_exercise(exercises: &[Exercise], previous_name: &mut String, cache: &mut StatusCache) {
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

            // Show file path (absolute)
            let abs_path = std::env::current_dir()
                .map(|cwd| cwd.join(&exercise.path))
                .unwrap_or_else(|_| exercise.path.clone());
            println!("  File: {}", abs_path.display().to_string().dimmed());

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
                            for line in output.lines().take(20) {
                                if line.contains("FAIL") || line.contains("panicked") {
                                    println!("  {}", line.red());
                                } else if line.contains("ok") {
                                    println!("  {}", line.green());
                                } else {
                                    println!("  {}", line);
                                }
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
            println!(
                "  {} seqlings hint",
                "Hint:".cyan()
            );
            show_progress(exercises, cache);
        }
        None => {
            // All done!
            clear_screen();
            println!("\n{}", "=".repeat(50).green());
            println!(
                "{}",
                "  Congratulations! You've completed all exercises!".green().bold()
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
    let mut cache = StatusCache::new();

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
                let header: Vec<&str> = content
                    .lines()
                    .take_while(|l| l.starts_with('#'))
                    .collect();
                for line in header {
                    println!("  {}", line.dimmed());
                }
            }

            println!();
            println!(
                "{}",
                "Open this file in your editor to complete the exercise.".yellow()
            );
            println!(
                "Run {} to see a hint.",
                "seqlings hint".cyan()
            );
            println!();

            // Open in $EDITOR if set
            if let Ok(editor) = std::env::var("EDITOR") {
                println!("Opening in {}...", editor.cyan());
                let cmd_status = process::Command::new(&editor)
                    .arg(&exercise.path)
                    .status();

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
                "Congratulations! You've completed all exercises!".green().bold()
            );
            show_progress(exercises, &mut cache);
        }
    }
}

/// List all exercises
fn cmd_list(exercises: &[Exercise]) {
    let mut cache = StatusCache::new();

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
}

/// Show hint for an exercise
fn cmd_hint(exercises: &[Exercise], name: Option<String>) {
    let mut cache = StatusCache::new();
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
                println!(
                    "\n{} {}",
                    "No hint available for".yellow(),
                    ex.name.cyan()
                );
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

/// Reset an exercise
fn cmd_reset(exercises: &[Exercise], name: Option<String>) {
    let mut cache = StatusCache::new();
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
            let solution_path = ex.solution_path();
            if solution_path.exists() {
                // Read the original (which we store inverted - solution has the answer)
                // For reset, we need the original broken version
                // TODO: Store originals separately, for now just add back # NOT DONE
                match std::fs::read_to_string(&ex.path) {
                    Ok(mut content) => {
                        if !content.contains("# NOT DONE") {
                            // Add marker back after the header comments
                            let insert_pos = content
                                .lines()
                                .take_while(|l| l.starts_with('#'))
                                .map(|l| l.len() + 1)
                                .sum::<usize>();
                            content.insert_str(insert_pos, "\n# NOT DONE\n");
                            if std::fs::write(&ex.path, content).is_ok() {
                                println!("{} {}", "Reset".green(), ex.name.cyan());
                            }
                        } else {
                            println!("{} is already in incomplete state", ex.name.cyan());
                        }
                    }
                    Err(e) => eprintln!("{} {}", "Error reading exercise:".red(), e),
                }
            } else {
                println!(
                    "{}",
                    "No original version found. Cannot reset.".yellow()
                );
            }
        }
        None => {
            eprintln!("{}", "Exercise not found".red());
        }
    }
}

/// Verify all exercises
fn cmd_verify(exercises: &[Exercise]) {
    let mut cache = StatusCache::new();

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
}

/// Skip to next exercise
fn cmd_next(exercises: &[Exercise]) {
    let mut cache = StatusCache::new();

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
}

/// Verify a single exercise and show result
#[allow(dead_code)]
fn verify_exercise(exercise: &Exercise) {
    let mut cache = StatusCache::new();
    let status = cache.get_status(exercise);
    println!(
        "{} {}",
        "Exercise status:".bold(),
        format_status(&status)
    );

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

    let bar_width = 30;
    let filled = (done * bar_width) / total;
    let empty = bar_width - filled;

    println!(
        "Progress: [{}{}] {}/{} ({}%)",
        "=".repeat(filled).green(),
        "-".repeat(empty),
        done,
        total,
        pct
    );
}
