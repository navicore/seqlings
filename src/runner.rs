//! Exercise compilation and test running

use std::path::Path;
use std::process::Command;

/// Compile a Seq file using seqc lint
///
/// Returns Ok(()) if compilation succeeds, Err with error message if it fails
pub fn compile(path: &Path) -> Result<(), String> {
    let output = Command::new("seqc")
        .arg("lint")
        .arg(path)
        .output()
        .map_err(|e| format!("Failed to run seqc: {}. Is seq installed and in PATH?", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Err(format!("{}{}", stdout, stderr))
    }
}

/// Run a Seq file's tests and return the output
///
/// Uses `seqc test` to run test functions (those named test-*).
/// The file is copied to a temp location with a `test-` prefix because
/// seqc test requires files to be named `test-*.seq`.
pub fn run_tests(path: &Path) -> Result<String, String> {
    // seqc test requires files named test-*.seq
    // Copy the exercise to a temp file with the right name
    let temp_dir = std::env::temp_dir();
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("exercise.seq");
    let temp_name = if file_name.starts_with("test-") {
        file_name.to_string()
    } else {
        format!("test-{}", file_name)
    };
    let temp_path = temp_dir.join(&temp_name);

    // Copy the file
    std::fs::copy(path, &temp_path)
        .map_err(|e| format!("Failed to copy exercise to temp file: {}", e))?;

    let output = Command::new("seqc")
        .arg("test")
        .arg(&temp_path)
        .output()
        .map_err(|e| format!("Failed to run seqc: {}. Is seq installed and in PATH?", e))?;

    // Clean up temp file
    let _ = std::fs::remove_file(&temp_path);

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(format!("{}{}", stdout, stderr))
    } else {
        // Return the output even on failure so we can show it
        Err(format!("{}{}", stdout, stderr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn temp_seq_file(content: &str) -> PathBuf {
        let dir = std::env::temp_dir();
        let path = dir.join("test_exercise.seq");
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    #[ignore] // Requires seqc to be installed
    fn test_compile_valid() {
        let path = temp_seq_file("42");
        assert!(compile(&path).is_ok());
        fs::remove_file(&path).ok();
    }

    #[test]
    #[ignore] // Requires seqc to be installed
    fn test_compile_invalid() {
        let path = temp_seq_file(": broken ( -- \n;"); // Invalid syntax
        assert!(compile(&path).is_err());
        fs::remove_file(&path).ok();
    }
}
