extern crate multiinput;

use multiinput::*;

use crate::letter_manager;
// https://github.com/obv-mikhail/InputBot maybe switch
pub struct Keymonitor {
    letter_manager: letter_manager::LetterManager,
}

impl Keymonitor {
    pub fn new() -> Self {
        let letter_manager: letter_manager::LetterManager = letter_manager::LetterManager::new();

        Keymonitor { letter_manager }
    }

    pub fn main_loop(&mut self) {
        let mut manager = RawInputManager::new().unwrap();
        manager.register_devices(DeviceType::Keyboards);
        'outer: loop {
            if let Some(event) = manager.get_event() {
                match event {
                    RawEvent::KeyboardEvent(_, KeyId::Escape, State::Pressed) => break 'outer,
                    RawEvent::KeyboardEvent(_, KeyId::Space, State::Pressed) => {
                        let word = self.letter_manager.make_word(" ".to_owned());
                        self.letter_manager.handle_word(word)
                    }
                    RawEvent::KeyboardEvent(_, KeyId::Return, State::Pressed) => {
                        let word = self.letter_manager.make_word("\n".to_owned());

                        self.letter_manager.handle_word(word)
                    }
                    RawEvent::KeyboardEvent(_, key, State::Pressed) => {
                        self.letter_manager.add_letter(self.key_id_to_string(&key))
                    }

                    _ => (),
                }
            }
        }
        println!("Finishing");
    }

    fn key_id_to_string(&self, key: &KeyId) -> String {
        match key {
            KeyId::Escape => "KC.ESC".to_string(),
            KeyId::Return => "\n".to_string(),
            KeyId::Backspace => "↚".to_string(),
            KeyId::Left => "←".to_string(),
            KeyId::Right => "→".to_string(),
            KeyId::Up => "↑".to_string(),
            KeyId::Down => "↓".to_string(),
            KeyId::Space => " ".to_string(),
            KeyId::A => "a".to_string(),
            KeyId::B => "b".to_string(),
            KeyId::C => "c".to_string(),
            KeyId::D => "d".to_string(),
            KeyId::E => "e".to_string(),
            KeyId::F => "f".to_string(),
            KeyId::G => "g".to_string(),
            KeyId::H => "h".to_string(),
            KeyId::I => "i".to_string(),
            KeyId::J => "j".to_string(),
            KeyId::K => "k".to_string(),
            KeyId::L => "l".to_string(),
            KeyId::M => "m".to_string(),
            KeyId::N => "n".to_string(),
            KeyId::O => "o".to_string(),
            KeyId::P => "p".to_string(),
            KeyId::Q => "q".to_string(),
            KeyId::R => "r".to_string(),
            KeyId::S => "s".to_string(),
            KeyId::T => "t".to_string(),
            KeyId::U => "u".to_string(),
            KeyId::V => "v".to_string(),
            KeyId::W => "w".to_string(),
            KeyId::X => "x".to_string(),
            KeyId::Y => "y".to_string(),
            KeyId::Z => "z".to_string(),
            KeyId::F1 => "F1".to_string(),
            KeyId::F2 => "F2".to_string(),
            KeyId::F3 => "F3".to_string(),
            KeyId::F4 => "F4".to_string(),
            KeyId::F5 => "F5".to_string(),
            KeyId::F6 => "F6".to_string(),
            KeyId::F7 => "F7".to_string(),
            KeyId::F8 => "F8".to_string(),
            KeyId::F9 => "F9".to_string(),
            KeyId::F10 => "F10".to_string(),
            KeyId::F11 => "F11".to_string(),
            KeyId::F12 => "F12".to_string(),
            KeyId::Zero => "0".to_string(),
            KeyId::One => "1".to_string(),
            KeyId::Two => "2".to_string(),
            KeyId::Three => "3".to_string(),
            KeyId::Four => "4".to_string(),
            KeyId::Five => "5".to_string(),
            KeyId::Six => "6".to_string(),
            KeyId::Seven => "7".to_string(),
            KeyId::Eight => "8".to_string(),
            KeyId::Nine => "9".to_string(),
            KeyId::Shift => "KC.SHIFT".to_string(),
            KeyId::LeftCtrl => "KC.LCTRL".to_string(),
            KeyId::RightCtrl => "KC.RCTRL".to_string(),
            KeyId::LeftAlt => "KC.LALT".to_string(),
            KeyId::RightAlt => "KC.RALT".to_string(),
            KeyId::CapsLock => "KC.CAPS".to_string(),
            KeyId::Pause => "KC.PAUSE".to_string(),
            KeyId::PageUp => "KC.PGUP".to_string(),
            KeyId::PageDown => "KC.PGDW".to_string(),
            KeyId::PrintScreen => "KC.PRNTSCRN".to_string(),
            KeyId::Insert => "KC.INS".to_string(),
            KeyId::End => "KC.END".to_string(),
            KeyId::Home => "KC.HOME".to_string(),
            KeyId::Delete => "KC.DEL".to_string(),
            KeyId::Add => "+".to_string(),
            KeyId::Subtract => "-".to_string(),
            KeyId::Multiply => "*".to_string(),
            KeyId::Separator => "|".to_string(),
            KeyId::Decimal => ".".to_string(),
            KeyId::Divide => "/".to_string(),
            KeyId::BackTick => "`".to_string(),
            KeyId::BackSlash => "\\".to_string(),
            KeyId::ForwardSlash => "/".to_string(),
            KeyId::Plus => "+".to_string(),
            KeyId::Minus => "-".to_string(),
            KeyId::FullStop => ".".to_string(),
            KeyId::Comma => ",".to_string(),
            KeyId::Tab => "KC.TAB".to_string(),
            KeyId::Numlock => "KC.NUM".to_string(),
            KeyId::LeftSquareBracket => "[".to_string(),
            KeyId::RightSquareBracket => "]".to_string(),
            KeyId::SemiColon => ";".to_string(),
            KeyId::Apostrophe => "'".to_string(),
            KeyId::Hash => "#".to_string(),
        }
    }
}
