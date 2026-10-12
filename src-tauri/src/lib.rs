mod commands;
mod models;
mod services;

use crate::commands::simplify::simplify_helper;
use crate::commands::truth_table::truth_table_helper;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![simplify_helper, truth_table_helper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
