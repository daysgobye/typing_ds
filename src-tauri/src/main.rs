// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_saver;
mod key_monitor; // Import the Keymonitor module from keyMonitor.rs
mod letter_manager;

fn main() {
    let mut keymonitor: key_monitor::Keymonitor = key_monitor::Keymonitor::new();
    // keymonitor.main_loop();
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
