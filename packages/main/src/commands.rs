use serde::Serialize;
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
pub struct CommandLine(Mutex<Option<Child>>);

#[derive(Serialize)]
pub struct CliCommand {
    program: String,
    arguments: Vec<String>,
}

#[derive(Serialize)]
pub struct WrappAction {
    name: String,
    command: CliCommand,
}

#[tauri::command]
pub fn get_actions(app: tauri::AppHandle, name: String) -> Result<Vec<WrappAction>, String> {
    let mut project_path = app.path_resolver().app_dir().unwrap();
    project_path.push("websites");
    project_path.push(&name);

    // TODO: Read project config and save to state instead
    let actions: Vec<WrappAction> = vec![WrappAction {
        name: "Start server".to_string(),
        command: CliCommand {
            program: "docker".to_string(),
            arguments: vec![
                "run".to_string(),
                "--rm".to_string(),
                "-it".to_string(),
                "-v".to_string(),
                "${pwd}:/usr/src/app".to_string(),
                "-p".to_string(),
                "5138:5138".to_string(),
                "test-vite".to_string(),
                "pnpm".to_string(),
                "run".to_string(),
                "dev".to_string(),
                "--host".to_string(),
                "--port".to_string(),
                "5138".to_string(),
            ],
        }, // command: "docker run --rm -it -v ${pwd}:/usr/src/app -p 5138:5138 test-vite pnpm run dev --host --port 5138".to_string(),
    }];

    Ok(actions)
}

#[tauri::command]
pub async fn start_action(
    app: tauri::AppHandle,
    command_line: tauri::State<'_, CommandLine>,
    name: String,
) -> Result<String, String> {
    let mut project_path = app.path_resolver().app_dir().unwrap();
    project_path.push("websites");
    project_path.push(&name);

    // TODO: Getactins from state instead
    let workspace = &project_path;
    let actions = get_actions(app, name).unwrap();
    let action = &actions[0];
    let command = &action.command;

    // Execute command
    // let mut command_parts = command.split(" ");

    let mut cli = Command::new(&command.program);
    cli.current_dir(&workspace);
    for arg in &command.arguments {
        cli.arg(arg.replace("${pwd}", &project_path.as_path().display().to_string()));
    }

    println!(
        "{}",
        String::from_utf8(cli.output().unwrap().stdout).unwrap()
    );

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
pub async fn stop_action(global_node: tauri::State<'_, CommandLine>) -> Result<(), String> {
    let mut node_changer = global_node.0.lock().unwrap();
    let node_opt = &mut *node_changer;
    if let Some(node) = node_opt {
        node.kill().unwrap();
    }
    *node_changer = None;
    Ok(())
}
