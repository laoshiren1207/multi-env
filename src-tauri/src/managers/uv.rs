use std::collections::BTreeMap;

use regex::Regex;

use crate::managers::EnvManager;
use crate::models::{RuntimeVersion, SwitchResult};
use crate::shell::ShellExecutor;

/// Manages Python versions via uv.
pub struct UvManager;

impl UvManager {
    pub fn new() -> Self {
        Self
    }
}

impl EnvManager for UvManager {
    fn name(&self) -> &str {
        "python"
    }

    fn display_name(&self) -> &str {
        "Python (uv)"
    }

    fn icon(&self) -> &str {
        "🐍"
    }

    fn manager_name(&self) -> &str {
        "uv"
    }

    fn is_available(&self) -> bool {
        ShellExecutor::run_command("uv --version").is_ok()
    }

    fn list_installed(&self) -> Result<Vec<RuntimeVersion>, String> {
        let output = ShellExecutor::run_command("uv python list 2>/dev/null")
            .map_err(|e| format!("Failed to list Python versions: {}", e))?;

        // Parse lines like:
        //   cpython-3.13.3-macos-aarch64-none  /opt/homebrew/bin/python3.13 -> ...
        //   cpython-3.12.10-macos-aarch64-none  /Users/xxx/.local/share/uv/python/...
        //   cpython-3.11.12-macos-aarch64-none  <download available>
        //
        // We only want lines with an actual path (not "<download available>").
        let re = Regex::new(r"^cpython-(\d+\.\d+\.\d+)-\S+\s+(/\S+)")
            .map_err(|e| format!("Regex error: {}", e))?;

        let current = self.current_version().ok().flatten();

        // Deduplicate by major.minor (keep the first / newest patch).
        let mut seen: BTreeMap<String, RuntimeVersion> = BTreeMap::new();

        for line in output.lines() {
            let line = line.trim();
            if line.contains("<download available>") {
                continue;
            }
            if let Some(caps) = re.captures(line) {
                let full_ver = caps.get(1).unwrap().as_str().to_string();
                let major_minor = full_ver
                    .splitn(3, '.')
                    .take(2)
                    .collect::<Vec<_>>()
                    .join(".");

                // Skip if we already have this major.minor
                if seen.contains_key(&major_minor) {
                    continue;
                }

                let is_current = current.as_deref().map_or(false, |c| c.starts_with(&major_minor));
                let display = format!("Python {}", major_minor);

                seen.insert(
                    major_minor,
                    RuntimeVersion {
                        version: full_ver,
                        is_current,
                        display_name: display,
                    },
                );
            }
        }

        // BTreeMap iterates in sorted key order; reverse for descending.
        let mut versions: Vec<RuntimeVersion> = seen.into_values().collect();
        versions.reverse();
        Ok(versions)
    }

    fn current_version(&self) -> Result<Option<String>, String> {
        let output = ShellExecutor::run_command("python3 --version 2>/dev/null")
            .unwrap_or_default();

        // "Python 3.13.3" -> "3.13.3"
        let version = output
            .strip_prefix("Python ")
            .map(|s| s.trim().to_string());

        Ok(version)
    }

    fn switch_to(&self, version: &str) -> Result<SwitchResult, String> {
        let from = self.current_version().unwrap_or(None);

        // Pin the version globally by writing to ~/.python-version
        // Both uv and pyenv respect ~/.python-version as the global default.
        let home = dirs::home_dir().unwrap_or_default();
        let py_version_file = home.join(".python-version");

        std::fs::write(&py_version_file, format!("{}\n", version.trim()))
            .map_err(|e| format!("Failed to write ~/.python-version: {}", e))?;

        Ok(SwitchResult {
            success: true,
            message: format!("已将全局 Python 版本切换为 {}", version),
            runtime: "python".to_string(),
            from_version: from,
            to_version: version.to_string(),
        })
    }

    fn install_version(&self, version: &str) -> Result<String, String> {
        let cmd = format!("uv python install {}", version.trim());
        let output = ShellExecutor::run_command(&cmd)
            .map_err(|e| format!("uv python install 失败: {}", e))?;
        Ok(format!("成功安装 Python {}: {}", version, output))
    }

    fn uninstall_version(&self, version: &str) -> Result<String, String> {
        let current = self.current_version().unwrap_or(None);
        if current.as_deref().map_or(false, |c| c.starts_with(version) || version.starts_with(c)) {
            return Err("无法卸载当前激活使用的 Python 版本，请先切换至其他版本！".to_string());
        }

        let cmd = format!("uv python uninstall {}", version.trim());
        let output = ShellExecutor::run_command(&cmd)
            .map_err(|e| format!("uv python uninstall 失败: {}", e))?;
        Ok(format!("已成功卸载 Python {}: {}", version, output))
    }
}
