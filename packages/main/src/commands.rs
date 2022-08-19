use std::fs;

#[tauri::command]
pub async fn get_websites(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let mut res: Vec<String> = Vec::new();

    let mut path = app.path_resolver().app_dir().unwrap();
    path.push("websites");

    match fs::read_dir(path) {
        Ok(dir) => {
            for ent_opt in dir {
                let ent = ent_opt.unwrap();
                if ent.file_type().unwrap().is_file() {
                    res.push(String::from(ent.file_name().to_str().unwrap()));
                }
            }

            return Ok(res);
        }
        Err(err) => {
          return Err(err.to_string());
        },
    }
}

#[tauri::command]
pub async fn add_website(app: tauri::AppHandle, name: String) {
    let mut path = app.path_resolver().app_dir().unwrap();
    path.push("websites");

    fs::create_dir_all(&path).unwrap();
    path.push(&name);

    fs::write(&path, "").unwrap();
}
