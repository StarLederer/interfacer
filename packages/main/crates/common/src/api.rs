use serde::Serialize;
use std::{
    path::Path,
    process::{Child, Command},
};

use crate::{git, state};

#[derive(Serialize)]
pub struct Consequence {
    pub name: String,
    pub active: bool,
}

fn use_child_process(
    command: &String,
    cwd: &Path,
    await_termination: bool,
) -> Option<Result<Child, String>> {
    let mut command_parts = command.split(" ");

    // Create command
    let mut command = Command::new(&command_parts.next().unwrap());
    command.current_dir(cwd);
    for arg in command_parts {
        // Apply simple preprocess
        // TODO: Instead find a way to execute Bash and other shell scripting languages
        command.arg(arg.replace("${pwd}", &cwd.display().to_string()));
    }

    // Spawn child
    match command.spawn() {
        Ok(mut child) => {
            if await_termination {
                child.wait().expect("");
                None
            } else {
                Some(Ok(child))
            }
        }
        Err(err) => Some(Err(err.to_string())),
    }
}

/*
    Mainly intended for internal use by interact().
    Performs the actual interaction on the &mut ActionState provided
*/
pub fn interact_directly(action: &mut state::ActionState, cwd: &Path) -> Result<Consequence, String> {
    if action.config.user_terminated {
        let name;

        // Check whether to start a new child or stop an existing one
        if let Some(child) = &mut action.process {
            // Stop the process
            match child.kill() {
                Ok(_) => {
                    action.process = None;
                    name = action.config.idle_name.clone();
                }
                Err(err) => {
                    return Err(err.to_string());
                }
            };
        } else {
            // Start the child process
            match use_child_process(&action.config.command, cwd, false).unwrap() {
                Ok(child) => {
                    action.process = Some(child);
                    name = action.config.active_name.clone();
                }
                Err(err) => return Err(err.to_string()),
            }
        }

        Ok(Consequence {
            name: name,
            active: action.process.is_some(),
        })
    } else {
        // Error out if action has a running process
        if action.process.is_some() {
            return Err("Action already active".to_string());
        }

        // Execute a child process
        use_child_process(&action.config.command, cwd, true);

        Ok(Consequence {
            name: action.config.idle_name.clone(),
            active: false,
        })
    }
}

/*
    API Endpoint.
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

pub fn get_user(state: &state::AppState) -> Option<String> {
    let b = state.user.as_ref()?;
    Some(b.local_name.clone())
}

/**
    This function is named the way it is because it does not have to
    be limited to git. Other version control software may be added in the future
*/
pub fn check_source_cloud(state: &state::AppState) -> Result<bool, String> {
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
