use std::{fs, sync::Mutex, vec};

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
pub struct AppState(Mutex<Option<common::state::AppState>>);

#[tauri::command]
pub fn load_project(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    name: String,
) -> Result<(), String> {
    let mut project_path = app.path_resolver().app_dir().unwrap();
    project_path.push("projects");
    project_path.push(&name);

    // Config
    let mut config_path = project_path.clone();
    config_path.push("wrapp.yaml");

    let config_src = match fs::read_to_string(&config_path) {
        Ok(src) => src,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    let config = match common::config::parse_config(
        &config_src,
        common::config::Config {
            version: String::from("1"),
            workspace_dir: String::from("./workspace"),
            after_code_download: vec![],
            before_code_upload: vec![],
            actions: vec![],
        },
    ) {
        Ok(config) => config,
        Err(err) => {
            return Err(err.to_string());
        }
    };

    *state.0.lock().unwrap() = Some(common::state::AppState::init(config, project_path));

    Ok(())
}

#[tauri::command]
pub fn get_actions(state: tauri::State<'_, AppState>) -> Result<Vec<String>, String> {
    let state = state.0.lock().unwrap();

    if let Some(state) = &*state {
        let mut action_names = Vec::new();
        for action in state.actions.iter() {
            action_names.push(action.config.idle_name.clone());
        }
        Ok(action_names)
    } else {
        Err("Attempted to start action before project is loaded".to_string())
    }
}

#[tauri::command]
pub async fn interact(
    action_i: usize,
    state: tauri::State<'_, AppState>,
) -> Result<common::api::Consequence, String> {
    let mut state = state.0.lock().unwrap();
    let state = match &mut *state {
        Some(state) => state,
        None => {
            return Err("Attempted to start action before project is loaded".to_string())
        },
    };

    if action_i >= (state.actions).len() {
        return Err("Action index out of bounds. This is a bug! Please report it".to_string());
    }

    let action = &mut state.actions[action_i];
    common::api::interact(action, &state.workspace_dir)
}
