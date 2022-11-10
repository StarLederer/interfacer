use serde::Serialize;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub mod git;
pub mod project_config;
pub mod state;
pub mod user_config;

#[cfg(not(test))]
mod internal;
#[cfg(test)]
pub mod internal;

#[cfg(test)]
mod tests;

#[derive(Serialize)]
pub struct Consequence {
    pub name: String,
    pub active: bool,
}

pub fn add_project(
    state: &state::AppState,
    app_dir: &Path,
    name: &str,
    git_url: &str,
    config: &project_config::ConfigLatest,
) -> Result<(), String> {
    // Directory

    let mut path = PathBuf::from(app_dir);
    path.push("projects");
    path.push(name);

    match fs::create_dir_all(&path) {
        Ok(_) => {}
        Err(err) => return Err(err.to_string()),
    };

    // Fail macro
    // Return it instead of returning an error
    macro_rules! fail {
        ($err: expr) => {
            // Remove the already created directory
            match fs::remove_dir_all(path) {
                // Removed successfully. Report that the addition gracefully failed
                Ok(_) => Err(String::from("Failed to add project (") + &$err.to_string() + ")"),

                // Faield to remove the already created project directory,
                // let the user know that they now have a corrupted project
                Err(err2) => Err(
                    String::from("Failed to add project (") +
                    &$err.to_string() +
                    "). On top of that, the project directory has already been created but failed to be removed (" +
                    &err2.to_string() +
                    "). Because of that the project will still show up in the list but will fail to load."
                ),
            }
        };
    }

    // Config

    let mut config_path = path.clone();
    config_path.push("config.yaml");

    let src = match serde_yaml::to_string(config) {
        Ok(src) => src,
        Err(err) => return fail!(err),
    };

    match fs::write(config_path, src) {
        Ok(_) => {}
        Err(err) => return fail!(err),
    };

    // Repo

    let user = state.user()?;

    let username = &user.git_username;
    let password = &user.git_password;

    let mut repo_path = path.clone();
    repo_path.push("workspace");

    match git::clone(git_url, &repo_path, username, password) {
        Ok(_) => {}
        Err(err) => return fail!(err),
    };

    // All clear

    Ok(())
}

/// Use when the user clicks an action.
pub fn interact(state: &mut state::AppState, action_i: usize) -> Result<Consequence, String> {
    let state = state.project_mut()?;

    if action_i >= (state.actions).len() {
        return Err("Action index out of bounds. This is a bug! Please report it".to_string());
    }

    let cwd = &state.workspace_dir;
    let action = &mut state.actions[action_i];

    internal::interact_directly(action, &cwd)
}

pub fn load_project(state: &mut state::AppState, app_dir: &Path, name: &str) -> Result<(), String> {
    let mut project_path = PathBuf::from(app_dir);
    project_path.push("projects");
    project_path.push(&name);

    let mut config_path = project_path.clone();
    config_path.push("config.yaml");

    match fs::read_to_string(&config_path) {
        Ok(config_src) => {
            match project_config::parse_config(
                &config_src,
                project_config::Config {
                    workspace_dir: String::from("./workspace"),
                    hooks: project_config::HookConfig {
                        before_each_action: vec![],
                        after_code_download: vec![],
                        before_code_upload: vec![],
                    },
                    actions: vec![],
                },
            ) {
                Ok(config) => match state::ProjectState::init(config, project_path) {
                    Ok(project_state) => {
                        state.set_project(project_state);
                        Ok(())
                    }
                    Err(err) => Err(err.to_string()),
                },
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn get_actions(state: &state::AppState) -> Result<Vec<Consequence>, String> {
    let state = state.project()?;

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
        Ok(src) => match user_config::parse_config(&src) {
            Ok(config) => match state::UserState::init(config) {
                Ok(user_state) => {
                    state.set_user(user_state);
                    Ok(true)
                }
                Err(err) => Err(err.to_string()),
            },
            Err(err) => Err(err.to_string()),
        },
        Err(err) => match err.kind() {
            // If there is no fonfig file return false
            std::io::ErrorKind::NotFound => Ok(false),
            // Error out if a different error occurs
            _ => Err(err.to_string()),
        },
    }
}

pub fn get_user(state: &state::AppState) -> Result<(), String> {
    state.user()?;
    Ok(())
}

pub fn set_user(state: &state::AppState) -> Result<(), String> {
    Err(String::from("Not implemented yet"))
}

// The following functions are named the way they are because they do not have to
// be limited to git. Other version control software may be added in the future

pub fn detect_local_source_changes(state: &state::AppState) -> Result<bool, String> {
    let project = state.project()?;

    let repo = &project.version_control.repo;

    match git::status(repo) {
        Ok(is_ahead) => Ok(is_ahead),
        Err(e) => Err(e.to_string()),
    }
}

pub fn detect_remote_source_changes(state: &state::AppState) -> Result<bool, String> {
    let project = state.project()?;
    let user = state.user()?;

    let repo = &project.version_control.repo;
    let remote = &project.version_control.remote;

    let username = &user.git_username;
    let password = &user.git_password;

    match git::fetch(repo, remote, username, password) {
        Ok(is_ahead) => Ok(is_ahead),
        Err(e) => Err(e.to_string()),
    }
}

pub fn download_remote_source_history(state: &state::AppState) -> Result<(), String> {
    let project = state.project()?;
    let user = state.user()?;

    let repo = &project.version_control.repo;
    let remote = &project.version_control.remote;

    let username = &user.git_username;
    let password = &user.git_password;

    match git::nuke_pull(repo, remote, username, password) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

/// Use when the user wants to "save" the project
///
/// # Returns
/// * **true** if uploaded successfully
/// * **false** if the user needs to downlaod first
///
/// # Todo
/// Return types are git specific.
/// We need to fix that if we want to support other
/// (remote) source control systems
pub fn upload_local_source_history(state: &state::AppState) -> Result<bool, String> {
    let project = state.project()?;
    let user = state.user()?;

    let repo = &project.version_control.repo;
    let remote = &project.version_control.remote;

    let username = &user.git_username;
    let password = &user.git_password;

    match git::add_all(repo) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string()),
    }

    match git::commit(repo) {
        Ok(_) => (),
        Err(e) => return Err(e.to_string()),
    }

    match git::push(repo, remote, username, password) {
        Ok(_) => Ok(true),
        Err(e) => match e.code() {
            git2::ErrorCode::NotFastForward => return Ok(false),
            _ => return Err(e.to_string()),
        },
    }
}
