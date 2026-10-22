use std::fs;

use crate::managers::sdkman::SdkmanManager;
use crate::managers::nvm::NvmManager;
use crate::managers::uv::UvManager;
use crate::managers::EnvManager;
use crate::models::{NodeMirrors, Profile, RuntimeInfo, SwitchResult};

/// 异步获取所有运行时信息，不阻塞 Tauri UI
#[tauri::command]
pub async fn get_all_runtimes() -> Result<Vec<RuntimeInfo>, String> {
    tokio::task::spawn_blocking(|| {
        let managers: Vec<Box<dyn EnvManager>> = vec![
            Box::new(SdkmanManager::new()),
            Box::new(NvmManager::new()),
            Box::new(UvManager::new()),
        ];

        let mut runtimes = Vec::new();
        for mgr in &managers {
            match mgr.get_info() {
                Ok(info) => runtimes.push(info),
                Err(e) => {
                    log::warn!("Failed to get info for {}: {}", mgr.name(), e);
                    runtimes.push(RuntimeInfo {
                        name: mgr.name().to_string(),
                        display_name: mgr.display_name().to_string(),
                        icon: mgr.icon().to_string(),
                        manager: mgr.manager_name().to_string(),
                        is_available: false,
                        current_version: None,
                        versions: Vec::new(),
                    });
                }
            }
        }

        Ok(runtimes)
    })
    .await
    .map_err(|e| format!("Task execution error: {}", e))?
}

/// 异步切换单个环境版本
#[tauri::command]
pub async fn switch_version(runtime: String, version: String) -> Result<SwitchResult, String> {
    tokio::task::spawn_blocking(move || {
        let manager: Box<dyn EnvManager> = match runtime.as_str() {
            "java" => Box::new(SdkmanManager::new()),
            "node" => Box::new(NvmManager::new()),
            "python" => Box::new(UvManager::new()),
            _ => return Err(format!("未知运行时: {}", runtime)),
        };

        if !manager.is_available() {
            return Err(format!(
                "{} ({}) 未安装或未找到",
                manager.display_name(),
                manager.manager_name()
            ));
        }

        manager.switch_to(&version)
    })
    .await
    .map_err(|e| format!("Task execution error: {}", e))?
}

/// 异步安装新版本
#[tauri::command]
pub async fn install_version(runtime: String, version: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let manager: Box<dyn EnvManager> = match runtime.as_str() {
            "java" => Box::new(SdkmanManager::new()),
            "node" => Box::new(NvmManager::new()),
            "python" => Box::new(UvManager::new()),
            _ => return Err(format!("未知运行时: {}", runtime)),
        };

        if !manager.is_available() {
            return Err(format!("{} 未安装", manager.display_name()));
        }

        manager.install_version(&version)
    })
    .await
    .map_err(|e| format!("Task execution error: {}", e))?
}

/// 异步卸载版本
#[tauri::command]
pub async fn uninstall_version(runtime: String, version: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let manager: Box<dyn EnvManager> = match runtime.as_str() {
            "java" => Box::new(SdkmanManager::new()),
            "node" => Box::new(NvmManager::new()),
            "python" => Box::new(UvManager::new()),
            _ => return Err(format!("未知运行时: {}", runtime)),
        };

        if !manager.is_available() {
            return Err(format!("{} 未安装", manager.display_name()));
        }

        manager.uninstall_version(&version)
    })
    .await
    .map_err(|e| format!("Task execution error: {}", e))?
}

/// 异步一键安装环境管理器 (SDKMAN, nvm, uv)
#[tauri::command]
pub async fn install_manager(manager: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let cmd = match manager.as_str() {
            "sdkman" => "curl -s \"https://get.sdkman.io\" | bash",
            "nvm" => "curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash",
            "uv" => "curl -LsSf https://astral.sh/uv/install.sh | sh",
            _ => return Err(format!("未知的环境管理器: {}", manager)),
        };

        log::info!("Installing manager {}: running {}", manager, cmd);
        let output = crate::shell::ShellExecutor::run_command(cmd)
            .map_err(|e| format!("安装 {} 失败: {}", manager, e))?;

        // 验证安装是否成功
        let home = dirs::home_dir().unwrap_or_default();
        let ok = match manager.as_str() {
            "sdkman" => home.join(".sdkman/bin/sdkman-init.sh").exists() || home.join(".sdkman/candidates/java").exists(),
            "nvm" => home.join(".nvm/nvm.sh").exists(),
            "uv" => home.join(".local/bin/uv").exists() || home.join(".cargo/bin/uv").exists() || std::path::Path::new("/opt/homebrew/bin/uv").exists() || crate::shell::ShellExecutor::run_command("export PATH=\"$HOME/.local/bin:$HOME/.cargo/bin:/opt/homebrew/bin:$PATH\" && uv --version").is_ok(),
            _ => false,
        };

        if ok {
            Ok(format!("{} 安装完成！", manager))
        } else {
            Ok(format!("安装命令已执行: {}", output))
        }
    })
    .await
    .map_err(|e| format!("任务执行失败: {}", e))?
}

// ── Node.js Mirrors Management (npm registry & nvm mirror) ─────────

/// 获取当前 npm registry 和 nvm node mirror
#[tauri::command]
pub async fn get_node_mirrors() -> Result<NodeMirrors, String> {
    tokio::task::spawn_blocking(|| {
        let home = dirs::home_dir().unwrap_or_default();

        // 1. 读取 npm registry
        let mut npm_registry = String::new();
        let npmrc_path = home.join(".npmrc");
        if npmrc_path.exists() {
            if let Ok(content) = fs::read_to_string(&npmrc_path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("registry=") || trimmed.starts_with("registry =") {
                        if let Some((_, val)) = trimmed.split_once('=') {
                            npm_registry = val.trim().to_string();
                            break;
                        }
                    }
                }
            }
        }
        if npm_registry.is_empty() {
            if let Ok(reg) = crate::shell::ShellExecutor::run_command("npm config get registry 2>/dev/null") {
                if !reg.is_empty() && !reg.contains("undefined") {
                    npm_registry = reg;
                }
            }
        }
        if npm_registry.is_empty() {
            npm_registry = "https://registry.npmjs.org/".to_string();
        }

        // 2. 读取 nvm mirror
        let mut nvm_mirror = String::new();
        let zshrc_path = home.join(".zshrc");
        if zshrc_path.exists() {
            if let Ok(content) = fs::read_to_string(&zshrc_path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("export NVM_NODEJS_ORG_MIRROR=") {
                        if let Some((_, val)) = trimmed.split_once('=') {
                            let val = val.trim().trim_matches('"').trim_matches('\'');
                            nvm_mirror = val.to_string();
                            break;
                        }
                    }
                }
            }
        }
        if nvm_mirror.is_empty() {
            nvm_mirror = std::env::var("NVM_NODEJS_ORG_MIRROR").unwrap_or_default();
        }
        if nvm_mirror.is_empty() {
            nvm_mirror = "https://nodejs.org/dist".to_string();
        }

        Ok(NodeMirrors {
            npm_registry,
            nvm_node_mirror: nvm_mirror,
        })
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

/// 设置 npm registry
#[tauri::command]
pub async fn set_npm_registry(registry: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let registry = registry.trim();
        if registry.is_empty() {
            return Err("镜像源地址不能为空".to_string());
        }

        let home = dirs::home_dir().unwrap_or_default();
        let npmrc_path = home.join(".npmrc");

        // 1. 同步写入 ~/.npmrc
        let mut new_lines = Vec::new();
        let mut replaced = false;
        if npmrc_path.exists() {
            if let Ok(content) = fs::read_to_string(&npmrc_path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("registry=") || trimmed.starts_with("registry =") {
                        new_lines.push(format!("registry={}", registry));
                        replaced = true;
                    } else {
                        new_lines.push(line.to_string());
                    }
                }
            }
        }
        if !replaced {
            new_lines.push(format!("registry={}", registry));
        }
        let _ = fs::write(&npmrc_path, new_lines.join("\n") + "\n");

        // 2. 通过 npm 命令行同步
        let cmd = format!("npm config set registry {}", registry);
        let _ = crate::shell::ShellExecutor::run_command(&cmd);

        Ok(format!("npm 镜像源已设置为: {}", registry))
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

/// 设置 nvm node 下载镜像 (NVM_NODEJS_ORG_MIRROR)
#[tauri::command]
pub async fn set_nvm_mirror(mirror: String) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let mirror = mirror.trim();
        if mirror.is_empty() {
            return Err("nvm 镜像地址不能为空".to_string());
        }

        let home = dirs::home_dir().unwrap_or_default();
        let zshrc_path = home.join(".zshrc");

        let mut lines = Vec::new();
        let mut replaced = false;
        if zshrc_path.exists() {
            if let Ok(content) = fs::read_to_string(&zshrc_path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("export NVM_NODEJS_ORG_MIRROR=") {
                        lines.push(format!("export NVM_NODEJS_ORG_MIRROR=\"{}\"", mirror));
                        replaced = true;
                    } else {
                        lines.push(line.to_string());
                    }
                }
            }
        }

        if !replaced {
            lines.push(format!("\n# NVM Node.js Mirror\nexport NVM_NODEJS_ORG_MIRROR=\"{}\"", mirror));
        }

        fs::write(&zshrc_path, lines.join("\n") + "\n")
            .map_err(|e| format!("写入 ~/.zshrc 失败: {}", e))?;

        // 设置当前进程环境变量
        unsafe {
            std::env::set_var("NVM_NODEJS_ORG_MIRROR", mirror);
        }

        Ok(format!("nvm 镜像已设置为: {}", mirror))
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

// ── Profile management ──────────────────────────────────────────────

fn profiles_path() -> std::path::PathBuf {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_default().join(".config"));
    config_dir.join("multi-env").join("profiles.json")
}

fn load_profiles() -> Result<Vec<Profile>, String> {
    let path = profiles_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let data = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read profiles: {}", e))?;
    serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse profiles: {}", e))
}

fn save_profiles(profiles: &[Profile]) -> Result<(), String> {
    let path = profiles_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
    }
    let data = serde_json::to_string_pretty(profiles)
        .map_err(|e| format!("Failed to serialize profiles: {}", e))?;
    fs::write(&path, data)
        .map_err(|e| format!("Failed to write profiles: {}", e))
}

#[tauri::command]
pub async fn get_profiles() -> Result<Vec<Profile>, String> {
    tokio::task::spawn_blocking(load_profiles)
        .await
        .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn save_profile(profile: Profile) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let mut profiles = load_profiles()?;
        if let Some(pos) = profiles.iter().position(|p| p.id == profile.id) {
            profiles[pos] = profile;
        } else {
            profiles.push(profile);
        }
        save_profiles(&profiles)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

#[tauri::command]
pub async fn delete_profile(id: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let mut profiles = load_profiles()?;
        profiles.retain(|p| p.id != id);
        save_profiles(&profiles)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}

/// 异步批量应用环境预设
#[tauri::command]
pub async fn apply_profile(id: String) -> Result<Vec<SwitchResult>, String> {
    tokio::task::spawn_blocking(move || {
        let profiles = load_profiles()?;
        let profile = profiles
            .iter()
            .find(|p| p.id == id)
            .ok_or_else(|| format!("未找到预设方案 '{}'", id))?;

        let mut results = Vec::new();

        // 切换 Java
        if let Some(ref ver) = profile.java_version {
            let mgr = SdkmanManager::new();
            if mgr.is_available() {
                match mgr.switch_to(ver) {
                    Ok(r) => results.push(r),
                    Err(e) => results.push(SwitchResult {
                        success: false,
                        message: e,
                        runtime: "java".to_string(),
                        from_version: None,
                        to_version: ver.clone(),
                    }),
                }
            }
        }

        // 切换 Node.js
        if let Some(ref ver) = profile.node_version {
            let mgr = NvmManager::new();
            if mgr.is_available() {
                match mgr.switch_to(ver) {
                    Ok(r) => results.push(r),
                    Err(e) => results.push(SwitchResult {
                        success: false,
                        message: e,
                        runtime: "node".to_string(),
                        from_version: None,
                        to_version: ver.clone(),
                    }),
                }
            }
        }

        // 切换 Python
        if let Some(ref ver) = profile.python_version {
            let mgr = UvManager::new();
            if mgr.is_available() {
                match mgr.switch_to(ver) {
                    Ok(r) => results.push(r),
                    Err(e) => results.push(SwitchResult {
                        success: false,
                        message: e,
                        runtime: "python".to_string(),
                        from_version: None,
                        to_version: ver.clone(),
                    }),
                }
            }
        }

        Ok(results)
    })
    .await
    .map_err(|e| format!("Task error: {}", e))?
}
