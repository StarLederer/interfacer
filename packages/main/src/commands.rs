use crate::config::*;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Child, Command},
    sync::Mutex,
};

#[tauri::command]
pub async fn get_websites(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let mut res: Vec<String> = Vec::new();

    let mut path = app.path_resolver().app_dir().unwrap();
    path.push("projects");

    match fs::read_dir(path) {
        Ok(dir) => {
            for ent_opt in dir {
                let ent = ent_opt.unwrap();
                if ent.file_type().unwrap().is_dir() {
                    res.push(String::from(ent.file_name().to_str().unwrap()));
                }
            }

            return Ok(res);
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

#[tauri::command]
pub async fn add_website(app: tauri::AppHandle, name: String, url: String) -> Result<(), String> {
    let mut path = app.path_resolver().app_dir().unwrap();
    path.push("projects");

    fs::create_dir_all(&path).unwrap();
    path.push(&name);

    match git2::Repository::clone(&url, path) {
        Ok(_) => {
            return Ok(());
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

#[derive(Default)]
pub struct CommandLine(Mutex<Option<Child>>);

#[derive(Default)]
pub struct Config(Mutex<Option<WrappConfig>>);

#[derive(Default)]
pub struct Workspace(Mutex<Option<PathBuf>>);

#[tauri::command]
pub fn load_project(
    app: tauri::AppHandle,
    config: tauri::State<'_, Config>,
    workspace: tauri::State<'_, Workspace>,
    name: String,
) -> Result<(), String> {
    let mut project_path = app.path_resolver().app_dir().unwrap();
    project_path.push("projects");
    project_path.push(&name);

    // Config
    let mut config_path = project_path.clone();
    config_path.push("wrapp.yaml");

    match fs::read_to_string(&config_path) {
        Ok(config_src) => match serde_yaml::from_str::<WrappConfig>(&config_src) {
            Ok(config_yaml) => {
                if let Some(workspace_path) = &config_yaml.workspace_dir {
                    *workspace.0.lock().unwrap() = Some(PathBuf::from(&workspace_path));
                } else {
                    let mut workspace_path = project_path.clone();
                    workspace_path.push("workspace");
                    *workspace.0.lock().unwrap() = Some(workspace_path);
                }
                *config.0.lock().unwrap() = Some(config_yaml);
                return Ok(());
            }
            Err(err) => {
                return Err(err.to_string());
            }
        },
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

#[tauri::command]
pub fn get_actions(config: tauri::State<'_, Config>) -> Result<Vec<WrappAction>, String> {
    let config_changer = config.0.lock().unwrap();
    let config_opt = &*config_changer;

    if let Some(config) = config_opt {
        return Ok(config.actions.clone());
    } else {
        return Err("Attempted to get actions before config is loaded".to_string());
    }
}

#[tauri::command]
pub async fn start_action(
    command_line: tauri::State<'_, CommandLine>,
    config: tauri::State<'_, Config>,
    workspace: tauri::State<'_, Workspace>,
    i: usize,
) -> Result<String, String> {
    // Get config
    let mut config_changer = config.0.lock().unwrap();
    let config_opt = &mut *config_changer;
    let config;
    if let Some(c) = config_opt {
        config = c
    } else {
        return Err("Attempted to start action before config is loaded".to_string());
    }

    // Get workspace
    let mut workspace_changer = workspace.0.lock().unwrap();
    let workspace_opt = &mut *workspace_changer;
    let workspace;
    if let Some(w) = workspace_opt {
        workspace = w
    } else {
        return Err("Attempted to start action before workspace is loaded".to_string());
    }

    // Get command
    let actions = &config.actions;
    let action = &actions[i];
    let command = &action.commands[0].command;

    // Execute command
    let mut command_parts = command.split(" ");

    let mut cli = Command::new(&command_parts.next().unwrap());
    cli.current_dir(&workspace);
    for arg in command_parts {
        cli.arg(arg.replace("${pwd}", &workspace.display().to_string()));
    }

    match cli.spawn() {
        Ok(child) => {
            *command_line.0.lock().unwrap() = Some(child);
            return Ok("Stop this action".to_string());
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

#[tauri::command]
pub fn stop_action(global_node: tauri::State<'_, CommandLine>) -> Result<(), String> {
    let mut node_changer = global_node.0.lock().unwrap();
    let node_opt = &mut *node_changer;
    if let Some(node) = node_opt {
        node.kill().unwrap();
    }
    *node_changer = None;
    Ok(())
}
