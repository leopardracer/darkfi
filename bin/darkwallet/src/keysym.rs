use miniquad::{
    KeyCode, MouseButton
};

pub trait KeyCodeAsStr {
    fn to_str(&self) -> &str;
}

impl KeyCodeAsStr for KeyCode {
    fn to_str(&self) -> &str {
        match self {
            Self::Space => " ",
            Self::Apostrophe => "'",
            Self::Comma => ",",
            Self::Minus => "-",
            Self::Period => ".",
            Self::Slash => "/",
            Self::Key0 => "0",
            Self::Key1 => "1",
            Self::Key2 => "2",
            Self::Key3 => "3",
            Self::Key4 => "4",
            Self::Key5 => "5",
            Self::Key6 => "6",
            Self::Key7 => "7",
            Self::Key8 => "8",
            Self::Key9 => "9",
            Self::Semicolon => ":",
            Self::Equal => "=",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
            Self::E => "E",
            Self::F => "F",
            Self::G => "G",
            Self::H => "H",
            Self::I => "I",
            Self::J => "J",
            Self::K => "K",
            Self::L => "L",
            Self::M => "M",
            Self::N => "N",
            Self::O => "O",
            Self::P => "P",
            Self::Q => "Q",
            Self::R => "R",
            Self::S => "S",
            Self::T => "T",
            Self::U => "U",
            Self::V => "V",
            Self::W => "W",
            Self::X => "X",
            Self::Y => "Y",
            Self::Z => "Z",
            Self::LeftBracket => "(",
            Self::Backslash => "\\",
            Self::RightBracket => ")",
            Self::GraveAccent => "GraveAccent",
            Self::World1 => "World1",
            Self::World2 => "World2",
            Self::Escape => "Escape",
            Self::Enter => "Enter",
            Self::Tab => "Tab",
            Self::Backspace => "Backspace",
            Self::Insert => "Insert",
            Self::Delete => "Delete",
            Self::Right => "Right",
            Self::Left => "Left",
            Self::Down => "Down",
            Self::Up => "Up",
            Self::PageUp => "PageUp",
            Self::PageDown => "PageDown",
            Self::Home => "Home",
            Self::End => "End",
            Self::CapsLock => "CapsLock",
            Self::ScrollLock => "ScrollLock",
            Self::NumLock => "NumLock",
            Self::PrintScreen => "PrintScreen",
            Self::Pause => "Pause",
            Self::F1 => "F1",
            Self::F2 => "F2",
            Self::F3 => "F3",
            Self::F4 => "F4",
            Self::F5 => "F5",
            Self::F6 => "F6",
            Self::F7 => "F7",
            Self::F8 => "F8",
            Self::F9 => "F9",
            Self::F10 => "F10",
            Self::F11 => "F11",
            Self::F12 => "F12",
            Self::F13 => "F13",
            Self::F14 => "F14",
            Self::F15 => "F15",
            Self::F16 => "F16",
            Self::F17 => "F17",
            Self::F18 => "F18",
            Self::F19 => "F19",
            Self::F20 => "F20",
            Self::F21 => "F21",
            Self::F22 => "F22",
            Self::F23 => "F23",
            Self::F24 => "F24",
            Self::F25 => "F25",
            Self::Kp0 => "Kp0",
            Self::Kp1 => "Kp1",
            Self::Kp2 => "Kp2",
            Self::Kp3 => "Kp3",
            Self::Kp4 => "Kp4",
            Self::Kp5 => "Kp5",
            Self::Kp6 => "Kp6",
            Self::Kp7 => "Kp7",
            Self::Kp8 => "Kp8",
            Self::Kp9 => "Kp9",
            Self::KpDecimal => "KpDecimal",
            Self::KpDivide => "KpDivide",
            Self::KpMultiply => "KpMultiply",
            Self::KpSubtract => "KpSubtract",
            Self::KpAdd => "KpAdd",
            Self::KpEnter => "KpEnter",
            Self::KpEqual => "KpEqual",
            Self::LeftShift => "LeftShift",
            Self::LeftControl => "LeftControl",
            Self::LeftAlt => "LeftAlt",
            Self::LeftSuper => "LeftSuper",
            Self::RightShift => "RightShift",
            Self::RightControl => "RightControl",
            Self::RightAlt => "RightAlt",
            Self::RightSuper => "RightSuper",
            Self::Menu => "Menu",
            Self::Unknown => "Unknown",
        }
    }
}

pub trait KeyCodeAsU16 {
    fn to_u16(&self) -> u16;
    fn from_u16(keysym: u16) -> Self;
}

impl KeyCodeAsU16 for KeyCode {
    fn to_u16(&self) -> u16 {
        match self {
            Self::Space => 0x0020,
            Self::Apostrophe => 0x0027,
            Self::Comma => 0x002c,
            Self::Minus => 0x002d,
            Self::Period => 0x002e,
            Self::Slash => 0x002f,
            Self::Key0 => 0x0030,
            Self::Key1 => 0x0031,
            Self::Key2 => 0x0032,
            Self::Key3 => 0x0033,
            Self::Key4 => 0x0034,
            Self::Key5 => 0x0035,
            Self::Key6 => 0x0036,
            Self::Key7 => 0x0037,
            Self::Key8 => 0x0038,
            Self::Key9 => 0x0039,
            Self::Semicolon => 0x003b,
            Self::Equal => 0x003d,
            Self::A => 0x0041,
            Self::B => 0x0042,
            Self::C => 0x0043,
            Self::D => 0x0044,
            Self::E => 0x0045,
            Self::F => 0x0046,
            Self::G => 0x0047,
            Self::H => 0x0048,
            Self::I => 0x0049,
            Self::J => 0x004a,
            Self::K => 0x004b,
            Self::L => 0x004c,
            Self::M => 0x004d,
            Self::N => 0x004e,
            Self::O => 0x004f,
            Self::P => 0x0050,
            Self::Q => 0x0051,
            Self::R => 0x0052,
            Self::S => 0x0053,
            Self::T => 0x0054,
            Self::U => 0x0055,
            Self::V => 0x0056,
            Self::W => 0x0057,
            Self::X => 0x0058,
            Self::Y => 0x0059,
            Self::Z => 0x005a,
            Self::LeftBracket => 0x005b,
            Self::Backslash => 0x005c,
            Self::RightBracket => 0x005d,
            Self::GraveAccent => 0x0060,
            Self::World1 => 0x0100,
            Self::World2 => 0x0101,
            Self::Escape => 0xff1b,
            Self::Enter => 0xff0d,
            Self::Tab => 0xff09,
            Self::Backspace => 0xff08,
            Self::Insert => 0xff63,
            Self::Delete => 0xffff,
            Self::Right => 0xff53,
            Self::Left => 0xff51,
            Self::Down => 0xff54,
            Self::Up => 0xff52,
            Self::PageUp => 0xff55,
            Self::PageDown => 0xff56,
            Self::Home => 0xff50,
            Self::End => 0xff57,
            Self::CapsLock => 0xffe5,
            Self::ScrollLock => 0xff14,
            Self::NumLock => 0xff7f,
            Self::PrintScreen => 0xfd1d,
            Self::Pause => 0xff13,
            Self::F1 => 0xffbe,
            Self::F2 => 0xffbf,
            Self::F3 => 0xffc0,
            Self::F4 => 0xffc1,
            Self::F5 => 0xffc2,
            Self::F6 => 0xffc3,
            Self::F7 => 0xffc4,
            Self::F8 => 0xffc5,
            Self::F9 => 0xffc6,
            Self::F10 => 0xffc7,
            Self::F11 => 0xffc8,
            Self::F12 => 0xffc9,
            Self::F13 => 0xffca,
            Self::F14 => 0xffcb,
            Self::F15 => 0xffcc,
            Self::F16 => 0xffcd,
            Self::F17 => 0xffce,
            Self::F18 => 0xffcf,
            Self::F19 => 0xffd0,
            Self::F20 => 0xffd1,
            Self::F21 => 0xffd2,
            Self::F22 => 0xffd3,
            Self::F23 => 0xffd4,
            Self::F24 => 0xffd5,
            Self::F25 => 0xffd6,
            Self::Kp0 => 0xffb0,
            Self::Kp1 => 0xffb1,
            Self::Kp2 => 0xffb2,
            Self::Kp3 => 0xffb3,
            Self::Kp4 => 0xffb4,
            Self::Kp5 => 0xffb5,
            Self::Kp6 => 0xffb6,
            Self::Kp7 => 0xffb7,
            Self::Kp8 => 0xffb8,
            Self::Kp9 => 0xffb9,
            Self::KpDecimal => 0xffae,
            Self::KpDivide => 0xffaf,
            Self::KpMultiply => 0xffaa,
            Self::KpSubtract => 0xffad,
            Self::KpAdd => 0xffab,
            Self::KpEnter => 0xff8d,
            Self::KpEqual => 0xffbd,
            Self::LeftShift => 0xffe1,
            Self::LeftControl => 0xffe3,
            Self::LeftAlt => 0xffe9,
            Self::LeftSuper => 0xffeb,
            Self::RightShift => 0xffe2,
            Self::RightControl => 0xffe4,
            Self::RightAlt => 0xffea,
            Self::RightSuper => 0xffec,
            Self::Menu => 0xff67,
            Self::Unknown => 0x01ff,
        }
    }
    fn from_u16(keysym: u16) -> Self {
        match keysym {
            0x0020 => Self::Space,
            0x0027 => Self::Apostrophe,
            0x002c => Self::Comma,
            0x002d => Self::Minus,
            0x002e => Self::Period,
            0x002f => Self::Slash,
            0x0030 => Self::Key0,
            0x0031 => Self::Key1,
            0x0032 => Self::Key2,
            0x0033 => Self::Key3,
            0x0034 => Self::Key4,
            0x0035 => Self::Key5,
            0x0036 => Self::Key6,
            0x0037 => Self::Key7,
            0x0038 => Self::Key8,
            0x0039 => Self::Key9,
            0x003b => Self::Semicolon,
            0x003d => Self::Equal,
            0x0041 => Self::A,
            0x0042 => Self::B,
            0x0043 => Self::C,
            0x0044 => Self::D,
            0x0045 => Self::E,
            0x0046 => Self::F,
            0x0047 => Self::G,
            0x0048 => Self::H,
            0x0049 => Self::I,
            0x004a => Self::J,
            0x004b => Self::K,
            0x004c => Self::L,
            0x004d => Self::M,
            0x004e => Self::N,
            0x004f => Self::O,
            0x0050 => Self::P,
            0x0051 => Self::Q,
            0x0052 => Self::R,
            0x0053 => Self::S,
            0x0054 => Self::T,
            0x0055 => Self::U,
            0x0056 => Self::V,
            0x0057 => Self::W,
            0x0058 => Self::X,
            0x0059 => Self::Y,
            0x005a => Self::Z,
            0x005b => Self::LeftBracket,
            0x005c => Self::Backslash,
            0x005d => Self::RightBracket,
            0x0060 => Self::GraveAccent,
            0x0100 => Self::World1,
            0x0101 => Self::World2,
            0xff1b => Self::Escape,
            0xff0d => Self::Enter,
            0xff09 => Self::Tab,
            0xff08 => Self::Backspace,
            0xff63 => Self::Insert,
            0xffff => Self::Delete,
            0xff53 => Self::Right,
            0xff51 => Self::Left,
            0xff54 => Self::Down,
            0xff52 => Self::Up,
            0xff55 => Self::PageUp,
            0xff56 => Self::PageDown,
            0xff50 => Self::Home,
            0xff57 => Self::End,
            0xffe5 => Self::CapsLock,
            0xff14 => Self::ScrollLock,
            0xff7f => Self::NumLock,
            0xfd1d => Self::PrintScreen,
            0xff13 => Self::Pause,
            0xffbe => Self::F1,
            0xffbf => Self::F2,
            0xffc0 => Self::F3,
            0xffc1 => Self::F4,
            0xffc2 => Self::F5,
            0xffc3 => Self::F6,
            0xffc4 => Self::F7,
            0xffc5 => Self::F8,
            0xffc6 => Self::F9,
            0xffc7 => Self::F10,
            0xffc8 => Self::F11,
            0xffc9 => Self::F12,
            0xffca => Self::F13,
            0xffcb => Self::F14,
            0xffcc => Self::F15,
            0xffcd => Self::F16,
            0xffce => Self::F17,
            0xffcf => Self::F18,
            0xffd0 => Self::F19,
            0xffd1 => Self::F20,
            0xffd2 => Self::F21,
            0xffd3 => Self::F22,
            0xffd4 => Self::F23,
            0xffd5 => Self::F24,
            0xffd6 => Self::F25,
            0xffb0 => Self::Kp0,
            0xffb1 => Self::Kp1,
            0xffb2 => Self::Kp2,
            0xffb3 => Self::Kp3,
            0xffb4 => Self::Kp4,
            0xffb5 => Self::Kp5,
            0xffb6 => Self::Kp6,
            0xffb7 => Self::Kp7,
            0xffb8 => Self::Kp8,
            0xffb9 => Self::Kp9,
            0xffae => Self::KpDecimal,
            0xffaf => Self::KpDivide,
            0xffaa => Self::KpMultiply,
            0xffad => Self::KpSubtract,
            0xffab => Self::KpAdd,
            0xff8d => Self::KpEnter,
            0xffbd => Self::KpEqual,
            0xffe1 => Self::LeftShift,
            0xffe3 => Self::LeftControl,
            0xffe9 => Self::LeftAlt,
            0xffeb => Self::LeftSuper,
            0xffe2 => Self::RightShift,
            0xffe4 => Self::RightControl,
            0xffea => Self::RightAlt,
            0xffec => Self::RightSuper,
            0xff67 => Self::Menu,
            _ => Self::Unknown,
        }
    }
}

pub trait MouseButtonAsU8 {
    fn to_u8(&self) -> u8;
    fn from_u8(btn: u8) -> Self;
}

impl MouseButtonAsU8 for MouseButton {
    fn to_u8(&self) -> u8 {
        match self {
            Self::Left => 0,
            Self::Middle => 1,
            Self::Right => 2,
            Self::Unknown => 3,
        }
    }

    fn from_u8(btn: u8) -> Self {
        match btn {
            0 => Self::Left,
            1 => Self::Middle,
            2 => Self::Right,
            _ => Self::Unknown,
        }
    }
}
