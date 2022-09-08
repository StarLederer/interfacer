#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod config;

use std::sync::Mutex;

use commands::*;

fn main() {
    tauri::Builder::default()
        .manage(Config::default())
        .manage(Workspace::default())
        .manage(Actions(Mutex::new(vec![
            common::Action {
                idle_name: "Start server".to_string(),
                active_name: "Stop server".to_string(),
                process: None,
                command: "ls -a".to_string(),
                user_terminated: true,
            },
            common::Action {
                idle_name: "Quick action".to_string(),
                active_name: "".to_string(),
                process: None,
                command: "ls -a".to_string(),
                user_terminated: false,
            },
            common::Action {
                idle_name: "Long action".to_string(),
                active_name: "".to_string(),
                process: None,
                command: "sleep 3".to_string(),
                user_terminated: false,
            },
        ])))
        .invoke_handler(tauri::generate_handler![
            add_website,
            get_websites,
            load_project,
            get_actions,
            interact,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
