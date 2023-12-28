// use tauri::api::path::app_config_dir;

use serde::{Deserialize, Serialize};

use crate::letter_manager::Utterance;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyLogFile {
    pub utterance: Vec<Utterance>,
}

pub fn save_utterance(utterance: Utterance, file_path: &String) {
    let new_path = Path::new(file_path);
    let data = KeyLogFile {
        utterance: vec![utterance; 1],
    };
    // let data: Vec<Utterance> = vec![utterance; 1];

    let yaml = toml::to_string(&data).expect("Cant Serialize Utterance");

    let mut fileRef = OpenOptions::new()
        .append(true)
        .create(true)
        .open(new_path.join("keylogger.toml"))
        .expect("Unable to open file");

    fileRef.write_all(yaml.as_bytes()).expect("write failed");
}
