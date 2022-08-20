#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

use commands::*;

fn main() {
    tauri::Builder::default()
        .manage(Node::default())
        .invoke_handler(tauri::generate_handler![
            add_website,
            get_websites,
            open_website,
            close_website
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
