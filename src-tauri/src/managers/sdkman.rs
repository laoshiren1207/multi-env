use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

use crate::managers::EnvManager;
use crate::models::{RuntimeVersion, SwitchResult};

/// Manages Java versions via SDKMAN.
pub struct SdkmanManager {
    sdkman_dir: PathBuf,
}

impl SdkmanManager {
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_default();
        Self {
            sdkman_dir: home.join(".sdkman"),
        }
    }

    fn candidates_dir(&self) -> PathBuf {
        self.sdkman_dir.join("candidates/java")
    }

    /// Read the current version by resolving the `current` symlink.
    fn read_current_symlink(&self) -> Option<String> {
        let current_link = self.candidates_dir().join("current");
        fs::read_link(&current_link)
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
    }
}

impl EnvManager for SdkmanManager {
    fn name(&self) -> &str {
        "java"
    }

    fn display_name(&self) -> &str {
        "Java (SDKMAN)"
    }

    fn icon(&self) -> &str {
        "☕"
    }

    fn manager_name(&self) -> &str {
        "sdkman"
    }

    fn is_available(&self) -> bool {
        self.sdkman_dir.join("candidates/java").exists()
            || self.sdkman_dir.join("bin/sdkman-init.sh").exists()
    }

    fn list_installed(&self) -> Result<Vec<RuntimeVersion>, String> {
        let java_dir = self.candidates_dir();
        if !java_dir.exists() {
            return Ok(Vec::new());
        }

        let current = self.read_current_symlink();

        let mut versions: Vec<RuntimeVersion> = fs::read_dir(&java_dir)
            .map_err(|e| format!("读取 SDKMAN Java 目录失败: {}", e))?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let name = entry.file_name().to_string_lossy().to_string();
                // Skip the `current` symlink itself
                if name == "current" {
                    return None;
                }
                // Only include directories
                if !entry.path().is_dir() {
                    return None;
                }
                let is_current = current.as_deref() == Some(&name);
                let major = extract_java_major(&name);
                let display = format!("Java {} ({})", major, name);
                Some(RuntimeVersion {
                    version: name,
                    is_current,
                    display_name: display,
                })
            })
            .collect();

        // Sort by major version number
        versions.sort_by(|a, b| {
            let a_major = extract_java_major_num(&a.version);
            let b_major = extract_java_major_num(&b.version);
            a_major.cmp(&b_major)
        });

        Ok(versions)
    }

    fn current_version(&self) -> Result<Option<String>, String> {
        Ok(self.read_current_symlink())
    }

    fn switch_to(&self, version: &str) -> Result<SwitchResult, String> {
        let from = self.read_current_symlink();

        // Verify target version directory exists
        let target_dir = self.candidates_dir().join(version);
        if !target_dir.exists() {
            return Err(format!("未找到已安装的 Java 版本: '{}'", version));
        }

        let current_link = self.candidates_dir().join("current");

        // 瞬间原子切换：移除旧 symlink 并创建指向新版本的 symlink
        if current_link.exists() || fs::symlink_metadata(&current_link).is_ok() {
            let _ = fs::remove_file(&current_link);
        }

        symlink(&target_dir, &current_link)
            .map_err(|e| format!("更新 SDKMAN current 软链接失败: {}", e))?;

        Ok(SwitchResult {
            success: true,
            message: format!("已将全局 Java 默认版本切换为 {}", version),
            runtime: "java".to_string(),
            from_version: from,
            to_version: version.to_string(),
        })
    }

    fn install_version(&self, version: &str) -> Result<String, String> {
        let init_script = self.sdkman_dir.join("bin/sdkman-init.sh").to_string_lossy().to_string();
        let cmd = format!("source {} && sdk install java {}", init_script, version.trim());
        let output = crate::shell::ShellExecutor::run_command(&cmd)
            .map_err(|e| format!("sdk install java 失败: {}", e))?;
        Ok(format!("成功安装 Java {}: {}", version, output))
    }

    fn uninstall_version(&self, version: &str) -> Result<String, String> {
        let current = self.read_current_symlink();
        if current.as_deref() == Some(version) {
            return Err("无法卸载当前激活使用的 Java 版本，请先切换至其他版本！".to_string());
        }

        let target_dir = self.candidates_dir().join(version);
        if !target_dir.exists() {
            return Err(format!("Java 版本 '{}' 不存在", version));
        }

        // 删除版本对应目录
        fs::remove_dir_all(&target_dir)
            .map_err(|e| format!("卸载 Java 目录失败: {}", e))?;

        Ok(format!("已成功卸载 Java {}", version))
    }
}

fn extract_java_major(identifier: &str) -> String {
    identifier
        .split('.')
        .next()
        .unwrap_or(identifier)
        .to_string()
}

fn extract_java_major_num(identifier: &str) -> u32 {
    identifier
        .split('.')
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0)
}
