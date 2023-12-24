// use tauri::api::path::app_config_dir;

use crate::letter_manager::Utterance;
use std::fs::OpenOptions;
use std::io::Write;

pub fn save_utterance(utterance: Utterance) {
    let data: Vec<Utterance> = vec![utterance; 1];

    let yaml = serde_yaml::to_string(&data).expect("Cant Serialize Utterance");

    let mut fileRef = OpenOptions::new()
        .append(true)
        .create(true)
        .open("keylogger.yaml")
        .expect("Unable to open file");

    fileRef.write_all(yaml.as_bytes()).expect("write failed");
}
