// Modules
mod commands;
mod events;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::launch_browser,
            commands::navigate_to_url,
            commands::get_page_content,
            commands::close_browser,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
