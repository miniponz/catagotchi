// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod pet;
mod system_tray;

use tauri::{Builder, Manager};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    // TODO: update to show a greeting from pet
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    Builder::default()
        .manage(pet::create_app_state().expect("Failed to create app state"))
        .setup(|app| {
            let app_handle = app.handle();
            system_tray::run_background_task(app_handle.clone());
            Ok(())
        })
        .system_tray(system_tray::create_system_tray())
        .on_system_tray_event(system_tray::handle_system_tray_event)
        .invoke_handler(tauri::generate_handler![
            pet::get_pet_state,
            pet::feed_pet,
            pet::check_bitcoin_blocks,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
