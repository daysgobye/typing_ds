// extern crate multiinput;

// use multiinput::*;

use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::letter_manager;
// https://github.com/obv-mikhail/InputBot maybe switch
pub struct Keymonitor {
    letter_manager: letter_manager::LetterManager,
    device_state: DeviceState,
    keys: Vec<Keycode>,
}

impl Keymonitor {
    pub fn new() -> Self {
        let device_state = DeviceState::new();
        let letter_manager: letter_manager::LetterManager = letter_manager::LetterManager::new();

        Keymonitor {
            letter_manager,
            device_state,
            keys: Vec::new(),
        }
    }

    pub fn main_loop(&mut self) {
        loop {
            let mut tmp_keys: Vec<Keycode> = Vec::new();

            let keys: Vec<Keycode> = self.device_state.get_keys();
            for key in keys.iter() {
                if !tmp_keys.contains(key) {
                    tmp_keys.push(key.to_owned());
                }
            }
            for old_key in self.keys.iter() {
                if !tmp_keys.contains(&old_key) {
                    match old_key {
                        Keycode::Space => {
                            let word = self.letter_manager.make_word(" ".to_owned());
                            self.letter_manager.handle_word(word)
                        }
                        Keycode::Enter => {
                            let word = self.letter_manager.make_word("\n".to_owned());
                            self.letter_manager.handle_word(word)
                        }

                        _ => self
                            .letter_manager
                            .add_letter(self.key_id_to_string(old_key)),
                    }
                }
            }
            self.keys = tmp_keys;
        }
    }

    fn key_id_to_string(&self, key: &Keycode) -> String {
        match key {
            Keycode::Escape => "KC.ESC".to_string(),
            Keycode::Backspace => "↚".to_string(),
            Keycode::Left => "←".to_string(),
            Keycode::Right => "→".to_string(),
            Keycode::Up => "↑".to_string(),
            Keycode::Down => "↓".to_string(),
            Keycode::Space => " ".to_string(),
            Keycode::A => "a".to_string(),
            Keycode::B => "b".to_string(),
            Keycode::C => "c".to_string(),
            Keycode::D => "d".to_string(),
            Keycode::E => "e".to_string(),
            Keycode::F => "f".to_string(),
            Keycode::G => "g".to_string(),
            Keycode::H => "h".to_string(),
            Keycode::I => "i".to_string(),
            Keycode::J => "j".to_string(),
            Keycode::K => "k".to_string(),
            Keycode::L => "l".to_string(),
            Keycode::M => "m".to_string(),
            Keycode::N => "n".to_string(),
            Keycode::O => "o".to_string(),
            Keycode::P => "p".to_string(),
            Keycode::Q => "q".to_string(),
            Keycode::R => "r".to_string(),
            Keycode::S => "s".to_string(),
            Keycode::T => "t".to_string(),
            Keycode::U => "u".to_string(),
            Keycode::V => "v".to_string(),
            Keycode::W => "w".to_string(),
            Keycode::X => "x".to_string(),
            Keycode::Y => "y".to_string(),
            Keycode::Z => "z".to_string(),
            Keycode::F1 => "F1".to_string(),
            Keycode::F2 => "F2".to_string(),
            Keycode::F3 => "F3".to_string(),
            Keycode::F4 => "F4".to_string(),
            Keycode::F5 => "F5".to_string(),
            Keycode::F6 => "F6".to_string(),
            Keycode::F7 => "F7".to_string(),
            Keycode::F8 => "F8".to_string(),
            Keycode::F9 => "F9".to_string(),
            Keycode::F10 => "F10".to_string(),
            Keycode::F11 => "F11".to_string(),
            Keycode::F12 => "F12".to_string(),
            Keycode::Key0 => "0".to_string(),
            Keycode::Key1 => "1".to_string(),
            Keycode::Key2 => "2".to_string(),
            Keycode::Key3 => "3".to_string(),
            Keycode::Key4 => "4".to_string(),
            Keycode::Key5 => "5".to_string(),
            Keycode::Key6 => "6".to_string(),
            Keycode::Key7 => "7".to_string(),
            Keycode::Key8 => "8".to_string(),
            Keycode::Key9 => "9".to_string(),
            Keycode::LShift => "KC.LSHIFT".to_string(),
            Keycode::RShift => "KC.RSHIFT".to_string(),
            Keycode::LControl => "KC.LCTRL".to_string(),
            Keycode::RControl => "KC.RCTRL".to_string(),
            Keycode::LAlt => "KC.LALT".to_string(),
            Keycode::RAlt => "KC.RALT".to_string(),
            Keycode::CapsLock => "KC.CAPS".to_string(),
            Keycode::PageUp => "KC.PGUP".to_string(),
            Keycode::PageDown => "KC.PGDW".to_string(),
            Keycode::Insert => "KC.INS".to_string(),
            Keycode::End => "KC.END".to_string(),
            Keycode::Home => "KC.HOME".to_string(),
            Keycode::Delete => "KC.DEL".to_string(),

            Keycode::Dot => ".".to_string(),
            Keycode::Slash => "/".to_string(),
            Keycode::BackSlash => "\\".to_string(),
            Keycode::Minus => "-".to_string(),
            Keycode::Comma => ",".to_string(),
            Keycode::Tab => "KC.TAB".to_string(),
            Keycode::LeftBracket => "[".to_string(),
            Keycode::RightBracket => "]".to_string(),
            Keycode::Semicolon => ";".to_string(),
            Keycode::Apostrophe => "'".to_string(),
            Keycode::Meta => "KC.GUI".to_string(),
            Keycode::Enter => "\n".to_string(),
            Keycode::Numpad0 => "0".to_string(),
            Keycode::Numpad1 => "1".to_string(),
            Keycode::Numpad2 => "2".to_string(),
            Keycode::Numpad3 => "3".to_string(),
            Keycode::Numpad4 => "4".to_string(),
            Keycode::Numpad5 => "5".to_string(),
            Keycode::Numpad6 => "6".to_string(),
            Keycode::Numpad7 => "7".to_string(),
            Keycode::Numpad8 => "8".to_string(),
            Keycode::Numpad9 => "9".to_string(),
            Keycode::NumpadSubtract => "-".to_string(),
            Keycode::NumpadAdd => "+".to_string(),
            Keycode::NumpadDivide => "/".to_string(),
            Keycode::NumpadMultiply => "*".to_string(),
            Keycode::Grave => "`".to_string(),
            Keycode::Equal => "=".to_string(),
        }
    }
}
