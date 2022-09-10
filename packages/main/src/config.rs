use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Command {
    lang: String,
    command: String,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub commands: Vec<Command>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub version: Option<String>,
    pub workspace_dir: Option<String>,
    pub after_code_download: Option<Vec<String>>,
    pub before_code_upload: Option<Vec<String>>,
    pub actions: Option<Vec<Action>>,
}

pub struct DefaultConfig {
    pub version: String,
    pub workspace_dir: String,
    pub after_code_download: Vec<String>,
    pub before_code_upload: Vec<String>,
    pub actions: Vec<Action>,
}

pub struct Config {
    pub version: String,
    pub workspace_dir: String,
    pub after_code_download: Vec<String>,
    pub before_code_upload: Vec<String>,
    pub actions: Vec<Action>,
}

pub fn parse_config(
    config_src: &String,
    defaults: &DefaultConfig
) -> Result<Config, serde_yaml::Error> {
    let config = serde_yaml::from_str::<UserConfig>(&config_src)?;

    Ok(Config {
        version: config.version.unwrap_or(defaults.version),
        workspace_dir: config.workspace_dir.unwrap_or(defaults.workspace_dir),
        after_code_download: config.after_code_download.unwrap_or(defaults.after_code_download),
        before_code_upload: config.before_code_upload.unwrap_or(defaults.before_code_upload),
        actions: config.actions.unwrap_or(defaults.actions),
    })
}
