#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod config;

use commands::*;

fn main() {
    tauri::Builder::default()
        .manage(CommandLine::default())
        .manage(Config::default())
        .manage(Workspace::default())
        .invoke_handler(tauri::generate_handler![
            add_website,
            get_websites,
            load_project,
            get_actions,
            start_action,
            stop_action
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
