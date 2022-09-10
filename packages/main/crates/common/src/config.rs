use serde::{Deserialize, Serialize};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

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

pub struct Config {
    pub version: String,
    pub workspace_dir: PathBuf,
    pub after_code_download: Vec<String>,
    pub before_code_upload: Vec<String>,
    pub actions: Vec<Action>,
}

pub fn parse_config(
    config_src: &String,
    defaults: &Config,
) -> Result<Config, serde_yaml::Error> {
    let config = serde_yaml::from_str::<UserConfig>(&config_src)?;

    Ok(Config {
        version: config.version.unwrap_or(defaults.version.clone()),
        workspace_dir: if config.workspace_dir.is_some() {
            PathBuf::from(config.workspace_dir.unwrap())
        } else {
            defaults.workspace_dir.clone()
        },
        after_code_download: config
            .after_code_download
            .unwrap_or(defaults.after_code_download.clone()),
        before_code_upload: config
            .before_code_upload
            .unwrap_or(defaults.before_code_upload.clone()),
        actions: config.actions.unwrap_or(defaults.actions.clone()),
    })
}
