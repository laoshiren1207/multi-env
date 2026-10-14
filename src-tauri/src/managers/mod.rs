pub mod sdkman;
pub mod nvm;
pub mod uv;

use crate::models::{RuntimeInfo, RuntimeVersion, SwitchResult};

/// Unified trait for all environment version managers.
pub trait EnvManager: Send + Sync {
    /// Internal runtime name (e.g. "java", "node", "python").
    fn name(&self) -> &str;

    /// Human-readable display name (e.g. "Java (SDKMAN)").
    fn display_name(&self) -> &str;

    /// Emoji icon for the runtime.
    fn icon(&self) -> &str;

    /// Manager identifier (e.g. "sdkman", "nvm", "uv").
    fn manager_name(&self) -> &str;

    /// Whether the underlying manager is installed and available.
    fn is_available(&self) -> bool;

    /// List all locally installed versions.
    fn list_installed(&self) -> Result<Vec<RuntimeVersion>, String>;

    /// Get the currently active version string, if any.
    fn current_version(&self) -> Result<Option<String>, String>;

    /// Switch to the specified version. Returns a result summary.
    fn switch_to(&self, version: &str) -> Result<SwitchResult, String>;

    /// Download and install a new version.
    fn install_version(&self, _version: &str) -> Result<String, String> {
        Err(format!("{} 暂不支持直接安装新版本", self.display_name()))
    }

    /// Uninstall an installed version.
    fn uninstall_version(&self, _version: &str) -> Result<String, String> {
        Err(format!("{} 暂不支持卸载功能", self.display_name()))
    }

    /// Convenience: build a full RuntimeInfo snapshot.
    fn get_info(&self) -> Result<RuntimeInfo, String> {
        let available = self.is_available();
        let (current, versions) = if available {
            (
                self.current_version().unwrap_or(None),
                self.list_installed().unwrap_or_default(),
            )
        } else {
            (None, Vec::new())
        };

        Ok(RuntimeInfo {
            name: self.name().to_string(),
            display_name: self.display_name().to_string(),
            icon: self.icon().to_string(),
            manager: self.manager_name().to_string(),
            is_available: available,
            current_version: current,
            versions,
        })
    }
}
