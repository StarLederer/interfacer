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
pub struct Config(Mutex<Option<common::config::Config>>);

#[derive(Default)]
pub struct Actions(pub Mutex<Vec<common::Action>>);

#[tauri::command]
pub fn load_project(
    app: tauri::AppHandle,
    config: tauri::State<'_, Config>,
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

    let mut workspace_path = project_path.clone();
    workspace_path.push("workspace");

    let parsed_config = match common::config::parse_config(
        &config_src,
        &common::config::Config {
            version: String::from("1"),
            workspace_dir: workspace_path,
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

    *config.0.lock().unwrap() = Some(parsed_config);

    Ok(())
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
    config: tauri::State<'_, Config>,
) -> Result<common::Consequence, String> {
    // Get actions
    let mut actions = actions.0.lock().unwrap();

    if action_i >= (*actions).len() {
        return Err("Action index out of bounds".to_string());
    }

    // Get config
    let config = config.0.lock().unwrap();
    let config = match &*config {
        Some(config) => config,
        None => {
            return Err("Attempted to start action before config is loaded".to_string());
        }
    };

    let action = &mut actions[action_i];
    common::interact(action, &config.workspace_dir)
}
