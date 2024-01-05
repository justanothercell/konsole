use crate::{input::{Command, ControlKey}, getch::Getch};

pub(crate) fn next_key(getch: &Getch) -> Result<Command, std::io::Error> {
    Ok(match getch.getch()? {
        3 => Command::CtrlC,
        0x7F => Command::Control { ctrl: false, key: ControlKey::Backspace },
        8 => Command::Control { ctrl: true, key: ControlKey::Backspace },
        b'\n' => Command::Enter,
        b'\t' => Command::Tab,
        0x1B => match getch.getch()? {
            b'[' => match getch.getch()? {
                b'A' => Command::Control { ctrl: false, key: ControlKey::Up },
                b'B' => Command::Control { ctrl: false, key: ControlKey::Down },
                b'C' => Command::Control { ctrl: false, key: ControlKey::Right },
                b'D' => Command::Control { ctrl: false, key: ControlKey::Left },
                b'H' => Command::Control { ctrl: false, key: ControlKey::Start },
                b'F' => Command::Control { ctrl: false, key: ControlKey::End },
                b'1' => match getch.getch()? {
                    b';' => match getch.getch()? {
                        b'5' => match getch.getch()? {
                            b'A' => Command::Control { ctrl: true, key: ControlKey::Up },
                            b'B' => Command::Control { ctrl: true, key: ControlKey::Down },
                            b'C' => Command::Control { ctrl: true, key: ControlKey::Right },
                            b'D' => Command::Control { ctrl: true, key: ControlKey::Left },
                            b'H' => Command::Control { ctrl: true, key: ControlKey::Start },
                            b'F' => Command::Control { ctrl: true, key: ControlKey::End },
                            c => Command::Unsupported(c)
                        }
                        c => Command::Unsupported(c)
                    }
                    c => Command::Unsupported(c)
                }
                b'3' => match getch.getch()? {
                    b'~' => Command::Control { ctrl: false, key: ControlKey::Delete },
                    b';' => match getch.getch()? {
                        b'5' => match getch.getch()? {
                            b'~' => Command::Control { ctrl: true, key: ControlKey::Delete },
                            c => Command::Unsupported(c)
                        }
                        c => Command::Unsupported(c)
                    }
                    c => Command::Unsupported(c)
                }
                b'5' => match getch.getch()? {
                    b'~' => Command::Control { ctrl: false, key: ControlKey::Top },
                    b';' => match getch.getch()? {
                        b'2' => match getch.getch()? {
                            b'~' => Command::Control { ctrl: true, key: ControlKey::Top },
                            c => Command::Unsupported(c)
                        }
                        c => Command::Unsupported(c)
                    }
                    c => Command::Unsupported(c)
                }
                b'6' => match getch.getch()? {
                    b'~' => Command::Control { ctrl: false, key: ControlKey::Bottom },
                    b';' => match getch.getch()? {
                        b'2' => match getch.getch()? {
                            b'~' => Command::Control { ctrl: true, key: ControlKey::Bottom },
                            c => Command::Unsupported(c)
                        }
                        c => Command::Unsupported(c)
                    }
                    c => Command::Unsupported(c)
                }
                c => Command::Unsupported(c)
            }
            c => Command::Unsupported(c)
        }
        c if c.is_ascii() && !c.is_ascii_control() => Command::Printable(c),
        c => Command::Unsupported(c)
    })
}
