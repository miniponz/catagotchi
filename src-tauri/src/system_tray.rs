use std::time::Duration;
use tauri::{AppHandle, Manager, SystemTray, SystemTrayMenu, SystemTrayEvent};

pub fn create_system_tray() -> SystemTray {
    let tray_menu = SystemTrayMenu::new(); 
    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
        }
        _ => {}
    }
}

pub fn run_background_task(app_handle: AppHandle) {
    std::thread::spawn(move || {
        loop {
            // Perform the Bitcoin block check
            let state = app_handle.state::<crate::pet::AppState>();
            if let Err(e) = crate::pet::check_bitcoin_blocks(state) {
                eprintln!("Error checking Bitcoin blocks: {}", e);
            }

            // Wait for 10 minutes before the next check
            std::thread::sleep(Duration::from_secs(600));
        }
    });
}