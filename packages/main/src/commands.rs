use std::{
    fs,
    process::{Child, Command},
    sync::Mutex,
};

#[tauri::command]
pub async fn get_websites(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let mut res: Vec<String> = Vec::new();

    let mut path = app.path_resolver().app_dir().unwrap();
    path.push("websites");

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
    path.push("websites");

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
pub struct Node(Mutex<Option<Child>>);

#[tauri::command]
pub async fn open_website(
    app: tauri::AppHandle,
    global_node: tauri::State<'_, Node>,
    name: String,
) -> Result<String, String> {
    let mut cwd = app.path_resolver().app_dir().unwrap();
    cwd.push("websites");
    cwd.push(&name);

    let mut env_path = cwd.clone();
    env_path.push("wrapp.env");
    let env_opt = envfile::EnvFile::new(&env_path);
    match env_opt {
        Ok(env) => {
            if let Some(port) = env.get("PORT") {
                match Command::new("node")
                    .arg("wrapp.launcher.js")
                    .current_dir(&cwd)
                    // .stdout(Stdio::piped())
                    .spawn()
                {
                    Ok(node) => {
                        *global_node.0.lock().unwrap() = Some(node);
                        let address = "http://localhost:".to_string() + port;
                        return Ok(address);
                    }
                    Err(err) => {
                        return Err(err.to_string());
                    }
                }
            } else {
                return Err("Failed to find PORT in wrapp.env".to_string());
            }
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}

#[tauri::command]
pub async fn close_website(global_node: tauri::State<'_, Node>) -> Result<(), String> {
    let mut node_changer = global_node.0.lock().unwrap();
    let node_opt = &mut *node_changer;
    if let Some(node) = node_opt {
      node.kill().unwrap();
    }
    *node_changer = None;
    Ok(())
}
