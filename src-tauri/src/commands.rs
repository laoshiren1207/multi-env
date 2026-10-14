use std::fs;

use crate::managers::sdkman::SdkmanManager;
use crate::managers::nvm::NvmManager;
use crate::managers::uv::UvManager;
use crate::managers::EnvManager;
use crate::models::{Profile, RuntimeInfo, SwitchResult};

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
