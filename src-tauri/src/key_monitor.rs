// extern crate multiinput;

// use multiinput::*;

use device_query::{DeviceQuery, DeviceState, Keycode};

use crate::letter_manager;
// https://github.com/obv-mikhail/InputBot maybe switch
pub struct Keymonitor {
    letter_manager: letter_manager::LetterManager,
    device_state: DeviceState,
    keys: Vec<Keycode>,
    shift: bool,
    ctrl: bool,
}

impl Keymonitor {
    pub fn new() -> Self {
        let device_state = DeviceState::new();
        let letter_manager: letter_manager::LetterManager = letter_manager::LetterManager::new();

        Keymonitor {
            letter_manager,
            device_state,
            keys: Vec::new(),
            shift: false,
            ctrl: false,
        }
    }

    pub fn main_loop_fn(&mut self) {
        let l_shift = Keycode::LShift;
        let r_shift = Keycode::RShift;
        let l_ctrl = Keycode::LControl;
        let r_ctrl = Keycode::RControl;

        let mut tmp_keys: Vec<Keycode> = Vec::new();

        let keys: Vec<Keycode> = self.device_state.get_keys();
        for key in keys.iter() {
            if !tmp_keys.contains(key) {
                tmp_keys.push(key.to_owned());
            }
        }

        if tmp_keys.contains(&l_shift) || tmp_keys.contains(&r_shift) {
            self.shift = true
        } else {
            self.shift = false
        }
        if tmp_keys.contains(&r_ctrl) || tmp_keys.contains(&l_ctrl) {
            self.ctrl = true
        } else {
            self.ctrl = false
        }
        for new_key in tmp_keys.iter() {
            if !self.keys.contains(&new_key) {
                match new_key {
                    Keycode::Space => {
                        let word = self.letter_manager.make_word(" ".to_owned());
                        self.letter_manager.handle_word(word)
                    }
                    Keycode::Enter => {
                        let word = self.letter_manager.make_word("\n".to_owned());
                        self.letter_manager.handle_word(word)
                    }
                    Keycode::Tab => {
                        let word = self.letter_manager.make_word("TAB".to_owned());
                        self.letter_manager.handle_word(word)
                    }
                    Keycode::RShift => {}
                    Keycode::LShift => {}
                    Keycode::RControl => {}
                    Keycode::LControl => {}

                    _ => self
                        .letter_manager
                        .add_letter(self.key_id_to_string(new_key)),
                }
            }
        }
        // for old_key in self.keys.iter() {
        //     if !tmp_keys.contains(&old_key) {
        //         match old_key {
        //             Keycode::Space => {
        //                 let word = self.letter_manager.make_word(" ".to_owned());
        //                 self.letter_manager.handle_word(word)
        //             }
        //             Keycode::Enter => {
        //                 let word = self.letter_manager.make_word("\n".to_owned());
        //                 self.letter_manager.handle_word(word)
        //             }

        //             _ => self
        //                 .letter_manager
        //                 .add_letter(self.key_id_to_string(old_key)),
        //         }
        //     }
        // }
        self.keys = tmp_keys;
    }

    fn key_id_to_string(&self, key: &Keycode) -> String {
        if self.ctrl && self.shift {
            match key {
                Keycode::Escape => "CTRL(ESC)".to_string(),
                Keycode::Backspace => "CTRL(↚)".to_string(),
                Keycode::Left => "CTRL(←)".to_string(),
                Keycode::Right => "CTRL(→)".to_string(),
                Keycode::Up => "CTRL(↑)".to_string(),
                Keycode::Down => "CTRL(↓)".to_string(),
                Keycode::Space => " ".to_string(),
                Keycode::A => "CTRL(A)".to_string(),
                Keycode::B => "CTRL(B)".to_string(),
                Keycode::C => "CTRL(C)".to_string(),
                Keycode::D => "CTRL(D)".to_string(),
                Keycode::E => "CTRL(E)".to_string(),
                Keycode::F => "CTRL(F)".to_string(),
                Keycode::G => "CTRL(G)".to_string(),
                Keycode::H => "CTRL(H)".to_string(),
                Keycode::I => "CTRL(I)".to_string(),
                Keycode::J => "CTRL(J)".to_string(),
                Keycode::K => "CTRL(K)".to_string(),
                Keycode::L => "CTRL(L)".to_string(),
                Keycode::M => "CTRL(M)".to_string(),
                Keycode::N => "CTRL(N)".to_string(),
                Keycode::O => "CTRL(O)".to_string(),
                Keycode::P => "CTRL(P)".to_string(),
                Keycode::Q => "CTRL(Q)".to_string(),
                Keycode::R => "CTRL(R)".to_string(),
                Keycode::S => "CTRL(S)".to_string(),
                Keycode::T => "CTRL(T)".to_string(),
                Keycode::U => "CTRL(U)".to_string(),
                Keycode::V => "CTRL(V)".to_string(),
                Keycode::W => "CTRL(W)".to_string(),
                Keycode::X => "CTRL(X)".to_string(),
                Keycode::Y => "CTRL(Y)".to_string(),
                Keycode::Z => "CTRL(Z)".to_string(),
                Keycode::F1 => "CTRL(F1)".to_string(),
                Keycode::F2 => "CTRL(F2)".to_string(),
                Keycode::F3 => "CTRL(F3)".to_string(),
                Keycode::F4 => "CTRL(F4)".to_string(),
                Keycode::F5 => "CTRL(F5)".to_string(),
                Keycode::F6 => "CTRL(F6)".to_string(),
                Keycode::F7 => "CTRL(F7)".to_string(),
                Keycode::F8 => "CTRL(F8)".to_string(),
                Keycode::F9 => "CTRL(F9)".to_string(),
                Keycode::F10 => "CTRL(F10)".to_string(),
                Keycode::F11 => "CTRL(F11)".to_string(),
                Keycode::F12 => "CTRL(F12)".to_string(),
                Keycode::Key0 => "CTRL())".to_string(),
                Keycode::Key1 => "CTRL(!)".to_string(),
                Keycode::Key2 => "CTRL(@)".to_string(),
                Keycode::Key3 => "CTRL(#)".to_string(),
                Keycode::Key4 => "CTRL($)".to_string(),
                Keycode::Key5 => "CTRL(%)".to_string(),
                Keycode::Key6 => "CTRL(^)".to_string(),
                Keycode::Key7 => "CTRL(&)".to_string(),
                Keycode::Key8 => "CTRL(*)".to_string(),
                Keycode::Key9 => "CTRL(()".to_string(),
                Keycode::LShift => "".to_string(),
                Keycode::RShift => "".to_string(),
                Keycode::LControl => "".to_string(),
                Keycode::RControl => "".to_string(),
                Keycode::LAlt => "".to_string(),
                Keycode::RAlt => "".to_string(),
                Keycode::CapsLock => "CAPS".to_string(),
                Keycode::PageUp => "PGUP".to_string(),
                Keycode::PageDown => "PGDW".to_string(),
                Keycode::Insert => "INS".to_string(),
                Keycode::End => "END".to_string(),
                Keycode::Home => "HOME".to_string(),
                Keycode::Delete => "DEL".to_string(),

                Keycode::Dot => "CTRL(>)".to_string(),
                Keycode::Slash => "CTRL(?)".to_string(),
                Keycode::BackSlash => "CTRL(|)".to_string(),
                Keycode::Minus => "CTRL(_)".to_string(),
                Keycode::Comma => "CTRL(<)".to_string(),
                Keycode::Tab => "TAB".to_string(),
                Keycode::LeftBracket => "CTRL({)".to_string(),
                Keycode::RightBracket => "CTRL(})".to_string(),
                Keycode::Semicolon => "CTRL(:)".to_string(),
                Keycode::Apostrophe => "\"".to_string(),
                Keycode::Meta => "GUI".to_string(),
                Keycode::Enter => "\n".to_string(),
                Keycode::Numpad0 => "CTRL(0)".to_string(),
                Keycode::Numpad1 => "CTRL(1)".to_string(),
                Keycode::Numpad2 => "CTRL(2)".to_string(),
                Keycode::Numpad3 => "CTRL(3)".to_string(),
                Keycode::Numpad4 => "CTRL(4)".to_string(),
                Keycode::Numpad5 => "CTRL(5)".to_string(),
                Keycode::Numpad6 => "CTRL(6)".to_string(),
                Keycode::Numpad7 => "CTRL(7)".to_string(),
                Keycode::Numpad8 => "CTRL(8)".to_string(),
                Keycode::Numpad9 => "CTRL(9)".to_string(),
                Keycode::NumpadSubtract => "-".to_string(),
                Keycode::NumpadAdd => "+".to_string(),
                Keycode::NumpadDivide => "/".to_string(),
                Keycode::NumpadMultiply => "*".to_string(),
                Keycode::Grave => "~".to_string(),
                Keycode::Equal => "+".to_string(),
            }
        } else if self.ctrl {
            match key {
                Keycode::Escape => "CTRL(ESC)".to_string(),
                Keycode::Backspace => "CTRL(↚)".to_string(),
                Keycode::Left => "CTRL(←)".to_string(),
                Keycode::Right => "CTRL(→)".to_string(),
                Keycode::Up => "CTRL(↑)".to_string(),
                Keycode::Down => "CTRL(↓)".to_string(),
                Keycode::Space => " ".to_string(),
                Keycode::A => "CTRL(a)".to_string(),
                Keycode::B => "CTRL(b)".to_string(),
                Keycode::C => "CTRL(c)".to_string(),
                Keycode::D => "CTRL(d)".to_string(),
                Keycode::E => "CTRL(e)".to_string(),
                Keycode::F => "CTRL(f)".to_string(),
                Keycode::G => "CTRL(g)".to_string(),
                Keycode::H => "CTRL(h)".to_string(),
                Keycode::I => "CTRL(i)".to_string(),
                Keycode::J => "CTRL(j)".to_string(),
                Keycode::K => "CTRL(k)".to_string(),
                Keycode::L => "CTRL(l)".to_string(),
                Keycode::M => "CTRL(m)".to_string(),
                Keycode::N => "CTRL(n)".to_string(),
                Keycode::O => "CTRL(o)".to_string(),
                Keycode::P => "CTRL(p)".to_string(),
                Keycode::Q => "CTRL(q)".to_string(),
                Keycode::R => "CTRL(r)".to_string(),
                Keycode::S => "CTRL(s)".to_string(),
                Keycode::T => "CTRL(t)".to_string(),
                Keycode::U => "CTRL(u)".to_string(),
                Keycode::V => "CTRL(v)".to_string(),
                Keycode::W => "CTRL(w)".to_string(),
                Keycode::X => "CTRL(x)".to_string(),
                Keycode::Y => "CTRL(y)".to_string(),
                Keycode::Z => "CTRL(z)".to_string(),
                Keycode::F1 => "CTRL(F1)".to_string(),
                Keycode::F2 => "CTRL(F2)".to_string(),
                Keycode::F3 => "CTRL(F3)".to_string(),
                Keycode::F4 => "CTRL(F4)".to_string(),
                Keycode::F5 => "CTRL(F5)".to_string(),
                Keycode::F6 => "CTRL(F6)".to_string(),
                Keycode::F7 => "CTRL(F7)".to_string(),
                Keycode::F8 => "CTRL(F8)".to_string(),
                Keycode::F9 => "CTRL(F9)".to_string(),
                Keycode::F10 => "CTRL(F10)".to_string(),
                Keycode::F11 => "CTRL(F11)".to_string(),
                Keycode::F12 => "CTRL(F12)".to_string(),
                Keycode::Key0 => "CTRL(0)".to_string(),
                Keycode::Key1 => "CTRL(1)".to_string(),
                Keycode::Key2 => "CTRL(2)".to_string(),
                Keycode::Key3 => "CTRL(3)".to_string(),
                Keycode::Key4 => "CTRL(4)".to_string(),
                Keycode::Key5 => "CTRL(5)".to_string(),
                Keycode::Key6 => "CTRL(6)".to_string(),
                Keycode::Key7 => "CTRL(7)".to_string(),
                Keycode::Key8 => "CTRL(8)".to_string(),
                Keycode::Key9 => "CTRL(9)".to_string(),
                Keycode::LShift => "LSHIFT".to_string(),
                Keycode::RShift => "RSHIFT".to_string(),
                Keycode::LControl => "LCTRL".to_string(),
                Keycode::RControl => "RCTRL".to_string(),
                Keycode::LAlt => "LALT".to_string(),
                Keycode::RAlt => "RALT".to_string(),
                Keycode::CapsLock => "CAPS".to_string(),
                Keycode::PageUp => "PGUP".to_string(),
                Keycode::PageDown => "PGDW".to_string(),
                Keycode::Insert => "INS".to_string(),
                Keycode::End => "END".to_string(),
                Keycode::Home => "HOME".to_string(),
                Keycode::Delete => "DEL".to_string(),

                Keycode::Dot => "CTRL(.)".to_string(),
                Keycode::Slash => "CTRL(/)".to_string(),
                Keycode::BackSlash => "CTRL(\\)".to_string(),
                Keycode::Minus => "CTRL(-)".to_string(),
                Keycode::Comma => "CTRL(,)".to_string(),
                Keycode::Tab => "CTRL(TAB)".to_string(),
                Keycode::LeftBracket => "CTRL([)".to_string(),
                Keycode::RightBracket => "CTRL(])".to_string(),
                Keycode::Semicolon => "CTRL(;)".to_string(),
                Keycode::Apostrophe => "CTRL(')".to_string(),
                Keycode::Meta => "CTRL(GUI)".to_string(),
                Keycode::Enter => "CTRL(\n)".to_string(),
                Keycode::Numpad0 => "CTRL(0)".to_string(),
                Keycode::Numpad1 => "CTRL(1)".to_string(),
                Keycode::Numpad2 => "CTRL(2)".to_string(),
                Keycode::Numpad3 => "CTRL(3)".to_string(),
                Keycode::Numpad4 => "CTRL(4)".to_string(),
                Keycode::Numpad5 => "CTRL(5)".to_string(),
                Keycode::Numpad6 => "CTRL(6)".to_string(),
                Keycode::Numpad7 => "CTRL(7)".to_string(),
                Keycode::Numpad8 => "CTRL(8)".to_string(),
                Keycode::Numpad9 => "CTRL(9)".to_string(),
                Keycode::NumpadSubtract => "-".to_string(),
                Keycode::NumpadAdd => "+".to_string(),
                Keycode::NumpadDivide => "/".to_string(),
                Keycode::NumpadMultiply => "*".to_string(),
                Keycode::Grave => "`".to_string(),
                Keycode::Equal => "=".to_string(),
            }
        } else if self.shift {
            match key {
                Keycode::Escape => "ESC".to_string(),
                Keycode::Backspace => "↚".to_string(),
                Keycode::Left => "←".to_string(),
                Keycode::Right => "→".to_string(),
                Keycode::Up => "↑".to_string(),
                Keycode::Down => "↓".to_string(),
                Keycode::Space => " ".to_string(),
                Keycode::A => "A".to_string(),
                Keycode::B => "B".to_string(),
                Keycode::C => "C".to_string(),
                Keycode::D => "D".to_string(),
                Keycode::E => "E".to_string(),
                Keycode::F => "F".to_string(),
                Keycode::G => "G".to_string(),
                Keycode::H => "H".to_string(),
                Keycode::I => "I".to_string(),
                Keycode::J => "J".to_string(),
                Keycode::K => "K".to_string(),
                Keycode::L => "L".to_string(),
                Keycode::M => "M".to_string(),
                Keycode::N => "N".to_string(),
                Keycode::O => "O".to_string(),
                Keycode::P => "P".to_string(),
                Keycode::Q => "Q".to_string(),
                Keycode::R => "R".to_string(),
                Keycode::S => "S".to_string(),
                Keycode::T => "T".to_string(),
                Keycode::U => "U".to_string(),
                Keycode::V => "V".to_string(),
                Keycode::W => "W".to_string(),
                Keycode::X => "X".to_string(),
                Keycode::Y => "Y".to_string(),
                Keycode::Z => "Z".to_string(),
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
                Keycode::Key0 => ")".to_string(),
                Keycode::Key1 => "!".to_string(),
                Keycode::Key2 => "@".to_string(),
                Keycode::Key3 => "#".to_string(),
                Keycode::Key4 => "$".to_string(),
                Keycode::Key5 => "%".to_string(),
                Keycode::Key6 => "^".to_string(),
                Keycode::Key7 => "&".to_string(),
                Keycode::Key8 => "*".to_string(),
                Keycode::Key9 => "(".to_string(),
                Keycode::LShift => "".to_string(),
                Keycode::RShift => "".to_string(),
                Keycode::LControl => "".to_string(),
                Keycode::RControl => "".to_string(),
                Keycode::LAlt => "".to_string(),
                Keycode::RAlt => "".to_string(),
                Keycode::CapsLock => "CAPS".to_string(),
                Keycode::PageUp => "PGUP".to_string(),
                Keycode::PageDown => "PGDW".to_string(),
                Keycode::Insert => "INS".to_string(),
                Keycode::End => "END".to_string(),
                Keycode::Home => "HOME".to_string(),
                Keycode::Delete => "DEL".to_string(),

                Keycode::Dot => ">".to_string(),
                Keycode::Slash => "?".to_string(),
                Keycode::BackSlash => "|".to_string(),
                Keycode::Minus => "_".to_string(),
                Keycode::Comma => "<".to_string(),
                Keycode::Tab => "TAB".to_string(),
                Keycode::LeftBracket => "{".to_string(),
                Keycode::RightBracket => "}".to_string(),
                Keycode::Semicolon => ":".to_string(),
                Keycode::Apostrophe => "\"".to_string(),
                Keycode::Meta => "GUI".to_string(),
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
                Keycode::Grave => "~".to_string(),
                Keycode::Equal => "+".to_string(),
            }
        } else {
            match key {
                Keycode::Escape => "ESC".to_string(),
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
                Keycode::LShift => "LSHIFT".to_string(),
                Keycode::RShift => "RSHIFT".to_string(),
                Keycode::LControl => "LCTRL".to_string(),
                Keycode::RControl => "RCTRL".to_string(),
                Keycode::LAlt => "LALT".to_string(),
                Keycode::RAlt => "RALT".to_string(),
                Keycode::CapsLock => "CAPS".to_string(),
                Keycode::PageUp => "PGUP".to_string(),
                Keycode::PageDown => "PGDW".to_string(),
                Keycode::Insert => "INS".to_string(),
                Keycode::End => "END".to_string(),
                Keycode::Home => "HOME".to_string(),
                Keycode::Delete => "DEL".to_string(),

                Keycode::Dot => ".".to_string(),
                Keycode::Slash => "/".to_string(),
                Keycode::BackSlash => "\\".to_string(),
                Keycode::Minus => "-".to_string(),
                Keycode::Comma => ",".to_string(),
                Keycode::Tab => "TAB".to_string(),
                Keycode::LeftBracket => "[".to_string(),
                Keycode::RightBracket => "]".to_string(),
                Keycode::Semicolon => ";".to_string(),
                Keycode::Apostrophe => "'".to_string(),
                Keycode::Meta => "GUI".to_string(),
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
}
