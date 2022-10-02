#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

use commands::*;

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            add_website,
            get_websites,
            load_project,
            load_user,
            get_actions,
            get_user,
            interact,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
