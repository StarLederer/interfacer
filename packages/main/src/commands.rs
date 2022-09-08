use crate::config::*;
use std::{fs, path::PathBuf, sync::Mutex};

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
pub struct Config(Mutex<Option<WrappConfig>>);

#[derive(Default)]
pub struct Workspace(Mutex<Option<PathBuf>>);

#[derive(Default)]
pub struct Actions(pub Mutex<Vec<common::Action>>);

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
pub fn get_actions(actions: tauri::State<'_, Actions>) -> Result<Vec<String>, String> {
    let config_changer = actions.0.lock().unwrap();
    let mut action_names = Vec::new();
    for action in (*config_changer).iter() {
        action_names.push(action.idle_name.clone());
    }
    Ok(action_names)
}

#[tauri::command]
pub async fn interact(
    action_i: usize,
    actions: tauri::State<'_, Actions>,
    workspace: tauri::State<'_, Workspace>,
) -> Result<common::Consequence, String> {
    // Get actions
    let mut actions = actions.0.lock().unwrap();

    if action_i >= (*actions).len() {
        return Err("Action index out of bounds".to_string());
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

    let action = &mut actions[action_i];
    common::interact(action, &workspace)
}
