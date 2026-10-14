use std::process::Command;

/// Executes shell commands via `/bin/zsh` and captures output.
pub struct ShellExecutor;

impl ShellExecutor {
    /// Run a raw shell command and return stdout.
    pub fn run_command(cmd: &str) -> Result<String, String> {
        let output = Command::new("/bin/zsh")
            .arg("-l")
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            // Some commands write useful info to stdout even on non-zero exit
            if !stdout.is_empty() && stderr.is_empty() {
                Ok(stdout)
            } else {
                Err(format!(
                    "Command failed (exit {}): {}",
                    output.status.code().unwrap_or(-1),
                    if stderr.is_empty() { &stdout } else { &stderr }
                ))
            }
        }
    }

    /// Run a command after sourcing an init script.
    /// e.g. `source ~/.nvm/nvm.sh && nvm current`
    #[allow(dead_code)]
    pub fn run_sourced_command(source_script: &str, cmd: &str) -> Result<String, String> {
        let full_cmd = format!("source {} && {}", source_script, cmd);
        Self::run_command(&full_cmd)
    }
}
