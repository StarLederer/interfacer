use crate::config::{self};
use std::{fs, path::PathBuf, process::Child};

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
    pub repo: git2::Repository,
}

impl AppState {
    pub fn init(config: config::Config, mut project_dir: PathBuf) -> Result<AppState, String> {
        // Init the actions vec
        let mut actions = vec![];
        for config in config.actions.iter() {
            actions.push(ActionState::from_config(config));
        }

        // Workspace dir
        project_dir.push(config.workspace_dir);
        let workspace_dir = match fs::canonicalize(project_dir) {
            Ok(some) => some,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        // Init the source-control repo
        let repo = match git2::Repository::open(&workspace_dir) {
            Ok(some) => some,
            Err(err) => {
                return Err(err.to_string());
            },
        };

        // Return
        Ok(AppState {
            workspace_dir,
            actions,
            repo
        })
    }
}
