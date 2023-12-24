use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Logger {
    tmp_log: HashMap<String, String>,
    time_range: u64,
    last_timestamp: u64,
    key: String,
    last_save: u64,
}

impl Logger {
    pub fn new(time_range: u64) -> Logger {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        Logger {
            tmp_log: HashMap::new(),
            time_range,
            last_timestamp: current_time,
            key: current_time.to_string(),
            last_save: current_time,
        }
    }

    pub fn save_word(&mut self, word: &str) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let current_key = current_time.to_string();

        if current_time - self.last_timestamp < self.time_range {
            self.key = current_key.clone();
        } else {
            self.key = current_key.clone();
            self.handle_save();
        }

        if let Some(entry) = self.tmp_log.get_mut(&current_key) {
            entry.push_str(word);
        } else {
            self.tmp_log.insert(current_key, word.to_string());
        }

        self.last_timestamp = current_time;
    }

    fn handle_save(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        if current_time - self.last_save > 10 {
            println!("saving file: {}", current_time - self.last_save);
            let json_object = serde_json::to_string_pretty(&self.tmp_log).unwrap();
            self.tmp_log.clear();
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("data.json")
                .expect("Failed to open data.json");
            file.write_all(json_object.as_bytes())
                .expect("Failed to write to file");
            file.write_all(b",\n").expect("Failed to write to file");
            self.last_save = current_time;
        } else {
            println!("it can wait: {}", current_time);
        }
    }
}
