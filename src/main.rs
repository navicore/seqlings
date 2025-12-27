//! Seqlings - Interactive exercises for learning Seq
//!
//! A rustlings-inspired tool for learning stack-based programming with Seq.

mod exercise;
mod runner;

use clap::{Parser, Subcommand};
use colored::Colorize;
use exercise::{Exercise, ExerciseStatus, load_exercises};
use std::process;

#[derive(Parser)]
#[command(name = "seqlings")]
#[command(version, about = "Interactive exercises for learning Seq")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List all exercises with their status
    List,
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
        Some(Commands::List) => cmd_list(&exercises),
        Some(Commands::Hint { name }) => cmd_hint(&exercises, name),
        Some(Commands::Reset { name }) => cmd_reset(&exercises, name),
        Some(Commands::Verify) => cmd_verify(&exercises),
        Some(Commands::Next) => cmd_next(&exercises),
        None => cmd_run(&exercises),
    }
}

/// Default command: find first incomplete exercise and open in editor
fn cmd_run(exercises: &[Exercise]) {
    // Find first incomplete exercise
    let current = exercises.iter().find(|e| {
        matches!(
            e.status(),
            ExerciseStatus::NotDone | ExerciseStatus::CompileError | ExerciseStatus::TestFail
        )
    });

    match current {
        Some(exercise) => {
            println!(
                "\n{} {}\n",
                "Current exercise:".green().bold(),
                exercise.name.cyan()
            );
            println!("  Path: {}", exercise.path.display());
            println!("  Status: {}", format_status(&exercise.status()));
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
                let status = process::Command::new(&editor)
                    .arg(&exercise.path)
                    .status();

                match status {
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
            show_progress(exercises);
        }
    }
}

/// List all exercises
fn cmd_list(exercises: &[Exercise]) {
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

        let status = exercise.status();
        let status_icon = match status {
            ExerciseStatus::Done => "✓".green(),
            ExerciseStatus::NotDone => "○".yellow(),
            ExerciseStatus::CompileError => "✗".red(),
            ExerciseStatus::TestFail => "✗".red(),
        };

        println!("    {} {}", status_icon, exercise.name);
    }

    println!();
    show_progress(exercises);
}

/// Show hint for an exercise
fn cmd_hint(exercises: &[Exercise], name: Option<String>) {
    let name_provided = name.is_some();
    let exercise = match &name {
        Some(n) => exercises.iter().find(|e| &e.name == n),
        None => exercises.iter().find(|e| {
            matches!(
                e.status(),
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
    let exercise = match name {
        Some(n) => exercises.iter().find(|e| e.name == n),
        None => exercises.iter().find(|e| {
            matches!(
                e.status(),
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
    println!("\n{}\n", "Verifying all exercises...".green().bold());

    for exercise in exercises {
        let status = exercise.status();
        let status_str = format_status(&status);
        let icon = match status {
            ExerciseStatus::Done => "✓".green(),
            _ => "✗".red(),
        };
        println!("  {} {} - {}", icon, exercise.name, status_str);
    }

    println!();
    show_progress(exercises);
}

/// Skip to next exercise
fn cmd_next(exercises: &[Exercise]) {
    // Find current incomplete
    let current_idx = exercises.iter().position(|e| {
        matches!(
            e.status(),
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
fn verify_exercise(exercise: &Exercise) {
    let status = exercise.status();
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

fn show_progress(exercises: &[Exercise]) {
    let done = exercises
        .iter()
        .filter(|e| matches!(e.status(), ExerciseStatus::Done))
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
