use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookConfigV1 {
    pub before_each_action: Option<Vec<String>>,
    pub after_code_download: Option<Vec<String>>,
    pub before_code_upload: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConfigV1 {
    pub command: String,
    pub user_terminated: bool,
    pub idle_name: String,
    pub active_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigV1 {
    pub version: Option<String>,
    pub workspace_dir: Option<String>,
    pub hooks: Option<HookConfigV1>,
    pub actions: Option<Vec<ActionConfigV1>>,
}
