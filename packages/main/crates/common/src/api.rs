use serde::Serialize;
use std::{
    path::Path,
    process::{Child, Command}
};

use crate::state;

#[derive(Serialize)]
pub struct Consequence {
    pub name: String,
    pub active: bool,
}

pub fn use_child_process(
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

pub fn interact(action: &mut state::ActionState, cwd: &Path) -> Result<Consequence, String> {
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
