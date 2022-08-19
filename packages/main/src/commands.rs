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
    let mut path = app.path_resolver().app_dir().unwrap();
    path.push("websites");
    path.push(&name);

    let output = Command::new("pnpm")
        .arg("install")
        .current_dir(&path)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8(output.stdout).unwrap());

    let output = Command::new("pnpm")
        .arg("run")
        .arg("dev")
        .current_dir(&path)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8(output.stdout).unwrap());

    tauri::WindowBuilder::new(
        &app,
        &name,
        tauri::WindowUrl::External("http://127.0.0.1:5173/".parse().unwrap()),
    )
    .build()
    .unwrap();
    Ok(())
}
