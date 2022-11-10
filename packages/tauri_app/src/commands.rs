use std::{fs, sync::Mutex};

use interfacer_core::project_config;

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
pub fn add_project(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    name: String,
    git_url: String,
    config: String,
) -> Result<(), String> {
    let config = serde_json::from_str::<project_config::ConfigLatest>(&config).expect("Failed to deserialize config");
    let app_dir = match app.path_resolver().app_dir() {
        Some(app_dir) => app_dir,
        None => return Err(String::from("Unable to determine the data directory")),
    };
    let state = state.0.lock().unwrap();
    interfacer_core::api::add_project(&state, &app_dir, &name, &git_url, &config)
}

#[derive(Default)]
pub struct AppState(Mutex<interfacer_core::state::AppState>);

#[tauri::command]
pub fn load_project(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    name: String,
) -> Result<(), String> {
    let app_dir = match app.path_resolver().app_dir() {
        Some(app_dir) => app_dir,
        None => return Err(String::from("Unable to determine the data directory")),
    };

    let mut state = state.0.lock().unwrap();
    interfacer_core::api::load_project(&mut state, &app_dir, &name)
}

#[tauri::command]
pub fn get_actions(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<interfacer_core::api::Consequence>, String> {
    let state = state.0.lock().unwrap();
    interfacer_core::api::get_actions(&state)
}

#[tauri::command]
pub fn load_user(app: tauri::AppHandle, state: tauri::State<'_, AppState>) -> Result<bool, String> {
    let app_dir = match app.path_resolver().app_dir() {
        Some(app_dir) => app_dir,
        None => return Err(String::from("Unable to determine the data directory")),
    };
    let mut state = state.0.lock().unwrap();
    interfacer_core::api::load_user(&mut state, &app_dir)
}

#[tauri::command]
pub fn get_user(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let state = state.0.lock().unwrap();
    interfacer_core::api::get_user(&state)
}

#[tauri::command]
pub fn set_user(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let state = state.0.lock().unwrap();
    interfacer_core::api::set_user(&state)
}

#[tauri::command]
pub async fn interact(
    state: tauri::State<'_, AppState>,
    action_i: usize,
) -> Result<interfacer_core::api::Consequence, String> {
    let mut state = state.0.lock().unwrap();
    interfacer_core::api::interact(&mut state, action_i)
}
