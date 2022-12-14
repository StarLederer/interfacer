#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;

fn main() {
    tauri::Builder::default()
        .manage(commands::AppState::default())
        .invoke_handler(tauri::generate_handler![
            commands::add_project,
            commands::get_websites,
            commands::load_project,
            commands::get_actions,
            commands::interact,
            commands::load_user,
            commands::get_user,
            commands::update_user,
            commands::detect_local_source_changes,
            commands::detect_remote_source_changes,
            commands::download_remote_source_history,
            commands::upload_local_source_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
