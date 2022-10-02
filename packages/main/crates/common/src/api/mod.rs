#[cfg(not(test))]
mod internal;
#[cfg(test)]
pub mod internal;

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{git, state, user_config, project_config};
use serde::Serialize;

use self::internal::interact_directly;

#[derive(Serialize)]
pub struct Consequence {
    pub name: String,
    pub active: bool,
}

/*
    Use when the user clicks an action.
*/
pub fn interact(state: &mut state::AppState, action_i: usize) -> Result<Consequence, String> {
    let state = match &mut state.project {
        Some(state) => state,
        None => {
            return Err(String::from(
                "Attempted to read project from state with no project",
            ))
        }
    };

    if action_i >= (state.actions).len() {
        return Err("Action index out of bounds. This is a bug! Please report it".to_string());
    }

    let cwd = &state.workspace_dir;
    let action = &mut state.actions[action_i];

    interact_directly(action, &cwd)
}

pub fn load_project(state: &mut state::AppState, app_dir: &Path, name: &str) -> Result<(), String> {
    let mut project_path = PathBuf::from(app_dir);
    project_path.push("projects");
    project_path.push(&name);

    let mut config_path = project_path.clone();
    config_path.push("wrapp.yaml");

    match fs::read_to_string(&config_path) {
        Ok(config_src) => {
            match project_config::parse_config(
                &config_src,
                project_config::Config {
                    version: String::from("1"),
                    workspace_dir: String::from("./workspace"),
                    after_code_download: vec![],
                    before_code_upload: vec![],
                    actions: vec![],
                },
            ) {
                Ok(config) => {
                    match state::ProjectState::init(config, project_path) {
                        Ok(project_state) => {
                            state.project = Some(project_state);
                            Ok(())
                        },
                        Err(err) => {
                            Err(err.to_string())
                        }
                    }
                },
                Err(err) => {
                    Err(err.to_string())
                }
            }
        },
        Err(err) => {
            Err(err.to_string())
        }
    }
}

pub fn get_actions(state: &state::AppState) -> Result<Vec<Consequence>, String> {
    let state = match &state.project {
        Some(state) => state,
        None => {
            return Err(String::from(
                "Attempted to read project state when no project is loaded",
            ))
        }
    };

    let mut consequences = Vec::new();

    for action in state.actions.iter() {
        consequences.push(Consequence {
            name: action.config.idle_name.clone(),
            active: action.process.is_some(),
        });
    }

    Ok(consequences)
}

pub fn load_user(state: &mut state::AppState, app_dir: &Path) -> Result<bool, String> {
    let mut config_path = PathBuf::from(app_dir);
    config_path.push("user.yaml");

    match fs::read_to_string(&config_path) {
        Ok(src) => {
            match user_config::parse_config(&src) {
                Ok(config) => {
                    match state::UserState::init(config) {
                        Ok(user_state) => {
                            state.user = Some(user_state);
                            Ok(true)
                        },
                        Err(err) => Err(err.to_string()),
                    }
                },
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => match err.kind() {
            // If there is no fonfig file return false
            std::io::ErrorKind::NotFound => Ok(false),
            // Error out if a different error occurs
            _ => Err(err.to_string()),
        },
    }
}

pub fn get_user(state: &state::AppState) -> Result<(), String> {
    match state.user {
        Some(_) => Ok(()),
        None => Err(String::from("Attempted to get user before user is loaded")),
    }
}

pub fn set_user(state: &state::AppState) -> Result<(), String> {
    Err(String::from("Not implemented yet"))
}

// The following functions are named the way they are because they do not have to
// be limited to git. Other version control software may be added in the future

pub fn detect_local_source_changes(state: &state::AppState) -> Result<bool, String> {
    let project = match &state.project {
        Some(project) => project,
        None => {
            return Err(String::from(
                "Attempted to read project from state with no project",
            ))
        }
    };

    let repo = &project.version_control.repo;

    match git::status(repo) {
        Ok(is_ahead) => Ok(is_ahead),
        Err(e) => Err(e.to_string()),
    }
}


pub fn detect_remote_source_changes(state: &state::AppState) -> Result<bool, String> {
    let project = match &state.project {
        Some(project) => project,
        None => {
            return Err(String::from(
                "Attempted to read project from state with no project",
            ))
        }
    };

    let user = match &state.user {
        Some(user) => user,
        None => {
            return Err(String::from(
                "Attempted to read user from state with no user",
            ))
        }
    };

    let repo = &project.version_control.repo;
    let remote = &project.version_control.remote;

    let username = &user.git_username;
    let password = &user.git_password;

    match git::fetch(repo, remote, username, password) {
        Ok(is_ahead) => Ok(is_ahead),
        Err(e) => Err(e.to_string()),
    }
}

// TODO: check source

// TODO: upload source
