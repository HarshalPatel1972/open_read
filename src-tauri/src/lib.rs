mod db;

use db::{init_db, search_dictionary, DbState};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Initialize database with app handle to access bundled resources
            let conn =
                init_db(Some(app.handle())).expect("Failed to initialize dictionary database");
            app.manage(DbState(std::sync::Mutex::new(conn)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![search_dictionary])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
