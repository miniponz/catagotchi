// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod pet;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .manage(pet::create_app_state().unwrap())
        .invoke_handler(tauri::generate_handler![
            pet::get_pet_state,
            pet::feed_pet,
            pet::check_bitcoin_blocks,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
