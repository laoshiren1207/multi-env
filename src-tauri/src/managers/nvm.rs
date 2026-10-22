use std::fs;
use std::path::PathBuf;

use crate::managers::EnvManager;
use crate::models::{RuntimeVersion, SwitchResult};

/// Manages Node.js versions via nvm.
pub struct NvmManager {
    nvm_dir: PathBuf,
}

impl NvmManager {
    pub fn new() -> Self {
        let home = dirs::home_dir().unwrap_or_default();
        let nvm_dir = std::env::var("NVM_DIR")
            .map(PathBuf::from)
            .unwrap_or_else(|_| home.join(".nvm"));
        Self { nvm_dir }
    }

    fn versions_dir(&self) -> PathBuf {
        self.nvm_dir.join("versions/node")
    }

    fn alias_dir(&self) -> PathBuf {
        self.nvm_dir.join("alias")
    }

    /// Read the default alias to determine the current global version.
    fn read_default_alias(&self) -> Option<String> {
        let alias_file = self.alias_dir().join("default");
        fs::read_to_string(&alias_file)
            .ok()
            .map(|s| s.trim().to_string())
    }

    /// Resolve an alias (e.g. "18" or "24" or "v18.20.8") to a full installed version (e.g. "v18.20.8").
    fn resolve_alias(&self, alias: &str) -> Option<String> {
        let versions_dir = self.versions_dir();
        if !versions_dir.exists() {
            return None;
        }

        let alias_clean = alias.strip_prefix('v').unwrap_or(alias);

        let mut matches: Vec<String> = fs::read_dir(&versions_dir)
            .ok()?
            .filter_map(|entry| {
                let name = entry.ok()?.file_name().to_string_lossy().to_string();
                let name_clean = name.strip_prefix('v').unwrap_or(&name);
                if name_clean.starts_with(alias_clean) {
                    Some(name)
                } else {
                    None
                }
            })
            .collect();

        matches.sort_by(|a, b| version_compare(a, b));
        matches.last().cloned()
    }
}

impl EnvManager for NvmManager {
    fn name(&self) -> &str {
        "node"
    }

    fn display_name(&self) -> &str {
        "Node.js (nvm)"
    }

    fn icon(&self) -> &str {
        "⬢"
    }

    fn manager_name(&self) -> &str {
        "nvm"
    }

    fn is_available(&self) -> bool {
        self.nvm_dir.join("nvm.sh").exists() || self.versions_dir().exists()
    }

    fn list_installed(&self) -> Result<Vec<RuntimeVersion>, String> {
        let versions_dir = self.versions_dir();
        if !versions_dir.exists() {
            return Ok(Vec::new());
        }

        let default_alias = self.read_default_alias();
        let resolved_default = default_alias
            .as_deref()
            .and_then(|a| self.resolve_alias(a));

        let mut versions: Vec<RuntimeVersion> = fs::read_dir(&versions_dir)
            .map_err(|e| format!("读取 nvm 版本目录失败: {}", e))?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let name = entry.file_name().to_string_lossy().to_string();
                if !entry.path().is_dir() || !name.starts_with('v') {
                    return None;
                }
                let is_current = resolved_default.as_deref() == Some(&name);
                let major = name
                    .strip_prefix('v')
                    .and_then(|s| s.split('.').next())
                    .unwrap_or("?");
                let display = format!("Node {}", major);
                Some(RuntimeVersion {
                    version: name,
                    is_current,
                    display_name: display,
                })
            })
            .collect();

        versions.sort_by(|a, b| version_compare(&a.version, &b.version));

        Ok(versions)
    }

    fn current_version(&self) -> Result<Option<String>, String> {
        let alias = self.read_default_alias();
        Ok(alias.as_deref().and_then(|a| self.resolve_alias(a)))
    }

    fn switch_to(&self, version: &str) -> Result<SwitchResult, String> {
        let from = self.current_version().unwrap_or(None);

        let version_str = if version.starts_with('v') {
            version.to_string()
        } else {
            format!("v{}", version)
        };

        let target_dir = self.versions_dir().join(&version_str);
        if !target_dir.exists() {
            return Err(format!("未找到已安装的 Node.js 版本: '{}'", version_str));
        }

        // 瞬间写入 default alias 文件，毫秒级生效！
        let alias_dir = self.alias_dir();
        if !alias_dir.exists() {
            let _ = fs::create_dir_all(&alias_dir);
        }

        let default_file = alias_dir.join("default");
        fs::write(&default_file, format!("{}\n", version_str))
            .map_err(|e| format!("写入 ~/.nvm/alias/default 失败: {}", e))?;

        Ok(SwitchResult {
            success: true,
            message: format!("已将全局 Node.js 默认版本切换为 {}", version_str),
            runtime: "node".to_string(),
            from_version: from,
            to_version: version_str,
        })
    }

    fn install_version(&self, version: &str) -> Result<String, String> {
        let mut mirror_env = String::new();
        let home = dirs::home_dir().unwrap_or_default();
        let zshrc_path = home.join(".zshrc");
        if let Ok(content) = fs::read_to_string(&zshrc_path) {
            for line in content.lines() {
                let trimmed = line.trim();
                if trimmed.starts_with("export NVM_NODEJS_ORG_MIRROR=") {
                    mirror_env = format!("{} && ", trimmed);
                    break;
                }
            }
        }
        if mirror_env.is_empty() {
            if let Ok(env_mirror) = std::env::var("NVM_NODEJS_ORG_MIRROR") {
                if !env_mirror.is_empty() {
                    mirror_env = format!("export NVM_NODEJS_ORG_MIRROR=\"{}\" && ", env_mirror);
                }
            }
        }

        let cmd = format!(
            "{}source {} && nvm install {}",
            mirror_env,
            self.nvm_dir.join("nvm.sh").to_string_lossy(),
            version.trim()
        );
        let output = crate::shell::ShellExecutor::run_command(&cmd)
            .map_err(|e| format!("nvm install 执行失败: {}", e))?;
        Ok(format!("成功安装 Node.js {}: {}", version, output))
    }

    fn uninstall_version(&self, version: &str) -> Result<String, String> {
        let current = self.current_version().unwrap_or(None);
        let version_str = if version.starts_with('v') {
            version.to_string()
        } else {
            format!("v{}", version)
        };

        if current.as_deref() == Some(&version_str) {
            return Err("无法卸载当前默认使用的 Node.js 版本，请先切换至其他版本！".to_string());
        }

        let target_dir = self.versions_dir().join(&version_str);
        if !target_dir.exists() {
            return Err(format!("Node.js 版本 '{}' 不存在", version_str));
        }

        fs::remove_dir_all(&target_dir)
            .map_err(|e| format!("卸载 Node.js 目录失败: {}", e))?;

        Ok(format!("已成功卸载 Node.js {}", version_str))
    }
}

fn version_compare(a: &str, b: &str) -> std::cmp::Ordering {
    let parse = |s: &str| -> Vec<u32> {
        s.strip_prefix('v')
            .unwrap_or(s)
            .split('.')
            .filter_map(|p| p.parse().ok())
            .collect()
    };
    let va = parse(a);
    let vb = parse(b);
    va.cmp(&vb)
}
