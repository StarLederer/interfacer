use std::{
    fs,
    process::{Command, Stdio},
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

#[tauri::command]
pub async fn open_website(app: tauri::AppHandle, name: String) -> Result<(), String> {
    let mut cwd = app.path_resolver().app_dir().unwrap();
    cwd.push("websites");
    cwd.push(&name);

    let mut env_path = cwd.clone();
    env_path.push("wrapp.env");
    let env_opt = envfile::EnvFile::new(&env_path);
    match env_opt {
        Ok(env) => {
            if let Some(port) = env.get("PORT") {
                Command::new("node")
                    .arg("wrapp.launcher.js")
                    .current_dir(&cwd)
                    .stdout(Stdio::piped())
                    .output()
                    .expect("Failed to execute wrapp.launcher.js");

                let address = "http://localhost:".to_string() + port;
                println!("{}", address);

                tauri::WindowBuilder::new(
                    &app,
                    "Editor".to_string(),
                    tauri::WindowUrl::External(
                        address.parse().expect("Failed to parse server address"),
                    ),
                )
                .build()
                .expect("Failed to build the editor window");
            } else {
                return Err("Failed to find PORT in wrapp.env".to_string());
            }
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }

    Ok(())
}
