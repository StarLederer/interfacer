use crate::{project_config, user_config};
use std::{fs, path::{PathBuf, Path}, process::Child};

pub struct ActionState {
    pub config: project_config::ActionConfig,
    pub process: Option<Child>,
}

impl ActionState {
    pub fn from_config(config: &project_config::ActionConfig) -> ActionState {
        ActionState {
            config: config.clone(),
            process: None,
        }
    }
}

pub struct VersionControlState {
    pub repo: git2::Repository,
    pub remote: String,
}

pub struct ProjectState {
    pub workspace_dir: PathBuf,
    pub actions: Vec<ActionState>,
    pub version_control: VersionControlState,
}

impl ProjectState {
    pub fn init(
        config: project_config::Config,
        mut project_dir: PathBuf,
    ) -> Result<ProjectState, String> {
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
                return Err(String::from("Workspace dir not found! ") + &err.to_string());
            }
        };

        let version_control = {
            // Init the source-control repo
            let repo = match git2::Repository::open(&workspace_dir) {
                Ok(some) => some,
                Err(err) => {
                    return Err(err.to_string());
                }
            };

            // Init version control state
            VersionControlState {
                repo,
                remote: String::from("Origin"),
            }
        };

        // Return
        Ok(ProjectState {
            workspace_dir,
            actions,
            version_control,
        })
    }
}

pub struct UserState {
    pub git_username: String,
    pub git_password: String,
}

impl UserState {
    pub fn init(config: user_config::Config) -> Result<UserState, String> {
        Ok(UserState {
            git_username: config.git.username,
            git_password: config.git.password,
        })
    }
}

#[derive(Default)]
pub struct AppState {
    pub project: Option<ProjectState>,
    pub user: Option<UserState>,
}
