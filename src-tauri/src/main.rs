// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;

mod file_saver;
mod key_monitor; // Import the Keymonitor module from keyMonitor.rs
mod letter_manager;

#[tauri::command]
async fn main_loop(save_path: String) {
    let mut keymonitor: key_monitor::Keymonitor = key_monitor::Keymonitor::new();
    loop {
        keymonitor.main_loop_fn()
    }
}

#[tauri::command]
async fn read_keylogger() -> String {
    println!("readding keylogger");
    let path = tauri::api::path::document_dir()
        .unwrap_or(std::path::PathBuf::new())
        .join("keylogger.yaml");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let deserialized_yaml: Vec<letter_manager::Utterance> =
        serde_yaml::from_str(&contents).expect("could not deserialized yaml");
    let json_string =
        serde_json::to_string(&deserialized_yaml).expect("Cant Serialize Utterance array");

    json_string
}
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![main_loop, read_keylogger])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
fn see_paths() {
    println!(
        "{}",
        tauri::api::path::data_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::local_data_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::cache_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::config_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::executable_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::public_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::runtime_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::template_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::font_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::home_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::audio_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::desktop_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::document_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::download_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
    println!(
        "{}",
        tauri::api::path::picture_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_string_lossy()
    );
}
