#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_help_command() {
        let output = Command::new("cargo")
            .args(["run", "--", "--help"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Anthropic CLI"));
        assert!(stdout.contains("models"));
        assert!(stdout.contains("chat"));
    }

    #[test]
    fn test_version_command() {
        let output = Command::new("cargo")
            .args(["run", "--", "--version"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("0.1.0"));
    }
}
