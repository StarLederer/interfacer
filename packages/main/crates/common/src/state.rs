use crate::config::{self};
use std::{path::PathBuf, process::Child};

pub struct ActionState {
    pub config: config::ActionConfig,
    pub process: Option<Child>,
}

impl ActionState {
    pub fn from_config(config: &config::ActionConfig) -> ActionState {
        ActionState {
            config: config.clone(),
            process: None,
        }
    }
}

pub struct AppState {
    pub workspace_dir: PathBuf,
    // pub after_code_download: Vec<ActionState>,
    // pub before_code_upload: Vec<ActionState>,
    pub actions: Vec<ActionState>,
}

impl AppState {
    pub fn init(config: config::Config, mut project_dir: PathBuf) -> AppState {
        let mut actions = vec![];
        for config in config.actions.iter() {
            actions.push(ActionState::from_config(config));
        }

        project_dir.push(config.workspace_dir);

        AppState {
            workspace_dir: project_dir,
            actions,
        }
    }
}
