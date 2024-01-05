use crate::{input::{Command, ControlKey}, getch::Getch};

pub(crate) fn next_key(getch: &Getch) -> Result<Command, std::io::Error> {
    Ok(match getch.getch()? {
        3 => Command::CtrlC,
        8 => Command::Control { ctrl: false, key: ControlKey::Backspace },
        0x7F => Command::Control { ctrl: true, key: ControlKey::Backspace },
        b'\n' | b'\r' => Command::Enter,
        b'\t' => Command::Tab,
        0xE0 => match getch.getch()? {
            b'H' => Command::Control { ctrl: false, key: ControlKey::Up },
            0x8D => Command::Control { ctrl: true, key: ControlKey::Up },
            b'P' => Command::Control { ctrl: false, key: ControlKey::Down },
            0x91 => Command::Control { ctrl: true, key: ControlKey::Down },
            b'K' => Command::Control { ctrl: false, key: ControlKey::Left },
            b's' => Command::Control { ctrl: true, key: ControlKey::Left },
            b'M' => Command::Control { ctrl: false, key: ControlKey::Right },
            b't' => Command::Control { ctrl: true, key: ControlKey::Right },
            b'G' => Command::Control { ctrl: false, key: ControlKey::Start },
            b'w' => Command::Control { ctrl: true, key: ControlKey::Start },
            b'O' => Command::Control { ctrl: false, key: ControlKey::End },
            b'u' => Command::Control { ctrl: true, key: ControlKey::End },
            b'S' => Command::Control { ctrl: false, key: ControlKey::Delete },
            0x7F => Command::Control { ctrl: true, key: ControlKey::Delete },
            b'I' => Command::Control { ctrl: false, key: ControlKey::Top },
            0x86 => Command::Control { ctrl: true, key: ControlKey::Top },
            b'Q' => Command::Control { ctrl: false, key: ControlKey::Bottom },
            b'v' => Command::Control { ctrl: true, key: ControlKey::Bottom },
            c => Command::Unsupported(c)
        }
        c if c.is_ascii() && !c.is_ascii_control() => Command::Printable(c),
        c => Command::Unsupported(c)
    })
}