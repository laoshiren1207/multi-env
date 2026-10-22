use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RuntimeVersion {
    pub version: String,
    pub is_current: bool,
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RuntimeInfo {
    pub name: String,
    pub display_name: String,
    pub icon: String,
    pub manager: String,
    pub is_available: bool,
    pub current_version: Option<String>,
    pub versions: Vec<RuntimeVersion>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SwitchResult {
    pub success: bool,
    pub message: String,
    pub runtime: String,
    pub from_version: Option<String>,
    pub to_version: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub java_version: Option<String>,
    pub node_version: Option<String>,
    pub python_version: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct NodeMirrors {
    pub npm_registry: String,
    pub nvm_node_mirror: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct AppState {
    pub profiles: Vec<Profile>,
}
