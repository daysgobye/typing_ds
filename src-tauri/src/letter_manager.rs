use crate::file_saver::save_utterance;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct LetterManager {
    pub letter_timestamps: Vec<(String, u64)>,
    pub words: Vec<WordInfo>,
    pub last_word: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WordInfo {
    pub word: String,
    pub letters_and_timestamps: Vec<(String, u64)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Utterance {
    pub words: String,
    pub word_info: Vec<WordInfo>,
}

impl LetterManager {
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        LetterManager {
            letter_timestamps: Vec::new(),
            words: Vec::new(),
            last_word: timestamp,
        }
    }

    pub fn handle_word(&mut self, word: WordInfo) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        self.words.push(word);

        if timestamp - self.last_word > 5 {
            let utterance = self.build_utterance();
            save_utterance(utterance)
        }
        self.last_word = timestamp
    }
    pub fn build_utterance(&mut self) -> Utterance {
        let mut words = String::new();
        let mut word_info = Vec::new();

        for word in &self.words {
            words.push_str(&word.word);
            let temp_word_info = WordInfo {
                word: word.word.clone(),
                letters_and_timestamps: word.letters_and_timestamps.clone(),
            };
            word_info.push(temp_word_info); // Add the WordInfo to the vector
        }

        self.words.clear(); // Clear the words vector

        Utterance { words, word_info }
    }

    pub fn add_letter(&mut self, letter: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        self.letter_timestamps.push((letter, timestamp));
    }

    pub fn make_word(&mut self, final_letter: String) -> WordInfo {
        let mut word = String::new();
        for (letter, _) in &self.letter_timestamps {
            if letter.to_owned() != "↚".to_string() {
                word.push_str(&letter);
            }
        }
        word.push_str(&final_letter);

        let word_info = WordInfo {
            word,
            letters_and_timestamps: self.letter_timestamps.clone(),
        };

        self.letter_timestamps.clear();
        word_info
    }
}
