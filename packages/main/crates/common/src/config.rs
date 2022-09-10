use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionConfig {
    pub command: String,
    pub user_terminated: bool,
    pub idle_name: String,
    pub active_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserConfig {
    version: Option<String>,
    workspace_dir: Option<String>,
    after_code_download: Option<Vec<String>>,
    before_code_upload: Option<Vec<String>>,
    actions: Option<Vec<ActionConfig>>,
}

pub struct Config {
    pub version: String,
    pub workspace_dir: String,
    pub after_code_download: Vec<String>,
    pub before_code_upload: Vec<String>,
    pub actions: Vec<ActionConfig>,
}

pub fn parse_config(
    config_src: &String,
    defaults: Config,
) -> Result<Config, serde_yaml::Error> {
    let config = serde_yaml::from_str::<UserConfig>(&config_src)?;

    Ok(Config {
        version: config
            .version
            .unwrap_or(defaults.version),
        workspace_dir: config
            .workspace_dir
            .unwrap_or(defaults.workspace_dir),
        after_code_download: config
            .after_code_download
            .unwrap_or(defaults.after_code_download),
        before_code_upload: config
            .before_code_upload
            .unwrap_or(defaults.before_code_upload),
        actions: config.actions.unwrap_or(defaults.actions),
    })
}
