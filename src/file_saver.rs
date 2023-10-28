use crate::letter_manager::Utterance;
use std::fs::OpenOptions;
use std::io::Write;

pub fn save_utterance(utterance: Utterance) {
    let yaml = serde_yaml::to_string(&utterance).expect("Cant Serialize Utterance");

    let mut fileRef = OpenOptions::new()
        .append(true)
        .create(true)
        .open("sample.yaml")
        .expect("Unable to open file");

    fileRef.write_all(yaml.as_bytes()).expect("write failed");
}
