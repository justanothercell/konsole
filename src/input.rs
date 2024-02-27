use std::io::Write;

use crate::{KONSOLE, SETTINGS, deactivate, Konsole, getch::Getch, tab_nothing, TabQuery, TabResult};

#[derive(Debug, PartialEq)]
pub(crate) enum ControlKey {
    Up, Down, Left, Right,
    Start, End,
    Top, Bottom,
    Backspace, Delete,
}

#[derive(Debug, PartialEq)]
pub(crate) enum Command {
    Control {
        ctrl: bool,
        key: ControlKey
    },
    Tab,
    Enter,
    CtrlC,
    Printable(u8),
    Unsupported(u8),
}


pub(crate) fn handle_input() {
    let getch = Getch::new();
    #[allow(clippy::while_immutable_condition)]
    while KONSOLE.lock().unwrap().running {
        refresh();
        let command = crate::system::next_key(&getch).expect("could nnot getch next char");
        let mut konsole = KONSOLE.lock().unwrap();
        if command != Command::Tab {
            konsole.tab_repeat = 0;
        }
        match command {
            Command::Control { ctrl: false, key: ControlKey::Start } => konsole.move_cursor(isize::MIN),
            Command::Control { ctrl: true, key: ControlKey::Start } => konsole.delete(isize::MIN),
            Command::Control { ctrl: false, key: ControlKey::End } => konsole.move_cursor(isize::MAX),
            Command::Control { ctrl: true, key: ControlKey::End } => konsole.delete(isize::MAX),
            Command::Control { ctrl, key: ControlKey::Backspace } => {
                let mut konsole = konsole;
                let w = if ctrl { -(konsole.to_boundary_left() as isize) } else { -1 };
                konsole.delete(w);
            }
            Command::Control { ctrl, key: ControlKey::Delete } => {
                let mut konsole = konsole;
                let w = if ctrl { konsole.to_boundary_right() as isize } else { 1 };
                konsole.delete(w);
            }
            Command::Control { ctrl, key: ControlKey::Left } => {
                let mut konsole = konsole;
                let w = if ctrl { -(konsole.to_boundary_left() as isize) } else { -1 };
                konsole.move_cursor(w);
            }
            Command::Control { ctrl, key: ControlKey::Right } => {
                let mut konsole = konsole;
                let w = if ctrl { konsole.to_boundary_right() as isize } else { 1 };
                konsole.move_cursor(w);
            }
            Command::Control { ctrl: _, key: ControlKey::Up } => konsole.history_up(),
            Command::Control { ctrl: _, key: ControlKey::Down } => konsole.history_down(),
            Command::Control { ctrl: _, key: _ } => todo!(),
            Command::Tab => {
                if konsole.tab_repeat == 0 {
                    konsole.cursor_before_tab = konsole.cursor
                }
                let mut settings = SETTINGS.lock().unwrap();
                let query = TabQuery {
                    input: konsole.input.clone(),
                    cursor_position: konsole.cursor,
                    tab_repeat: konsole.tab_repeat,
                    cursor_before: konsole.cursor_before_tab
                };
                let tab_complete = settings.tab_complete.clone();
                konsole.tab_repeat += 1;
                drop(konsole);
                drop(settings);
                if let Some(TabResult { output, cursor_movement }) = tab_complete(query) {
                    let mut konsole = KONSOLE.lock().unwrap();
                    konsole.input = output;
                    konsole.move_cursor(cursor_movement);
                }
            },
            Command::Enter => konsole.submit(),
            Command::CtrlC => { std::thread::spawn(|| deactivate(None)); break; },
            Command::Printable(c) => konsole.add_char(c),
            Command::Unsupported(_) => (),
        }
    }
}

impl Konsole {
    // note: some clones in this function used to be std::mem::take, but that caused UB/segfaults for an unknown reason
    fn submit(&mut self) {
        let input = self.input.clone();
        self.input.clear();
        self.cursor = 0;
        self.temp_input = None;
        let settings = SETTINGS.lock().unwrap();
        if settings.history_enabled && !input.is_empty() && self.history.get(0).map(|h| h != &input).unwrap_or(true){
            if self.history.len() > settings.history_limit {
                self.history.pop_back();
            }
            self.history.push_front(input.clone());
            self.history_index = 0;

        }
        self.queued_inputs.push(input);
    }

    // note: some clones in this function used to be std::mem::take, but that caused UB/segfaults for an unknown reason
    fn history_up(&mut self) {
        let settings = SETTINGS.lock().unwrap();
        if !settings.history_enabled { return; }
        if self.history_index >= settings.history_limit || self.history_index >= self.history.len() { return; }
        // first move? then let's store the current input
        if self.temp_input.is_none() {
            self.temp_input = Some(self.input.clone());
        } else if self.history_index < settings.history_limit - 1 && self.history_index < self.history.len() - 1 { 
            self.history_index += 1;
        }
        self.input = self.history[self.history_index].clone();
        self.cursor = self.input.len();
        
    }

    // note: some clones in this function used to be std::mem::take, but that caused UB/segfaults for an unknown reason
    fn history_down(&mut self) {
        let settings = SETTINGS.lock().unwrap();
        if !settings.history_enabled { return; }
        if self.history.is_empty() { return; }
        if self.history_index == 0 {
            if let Some(temp) = self.temp_input.take() {
                self.input = temp;
            }
        } else {
            self.history_index -= 1;
            self.input = self.history[self.history_index].clone();
        }
        self.cursor = self.input.len();
    }

    fn ensure_take_history(&mut self) {
        self.history_index = 0;
        self.temp_input = None;
    }

    fn add_char(&mut self, c: u8) {
        self.ensure_take_history();
        self.input.insert(self.cursor, c as char);
        self.cursor += 1;
    }

    fn delete(&mut self, amount: isize) {
        self.ensure_take_history();
        if amount < 0 {
            for _ in 0..amount.saturating_neg() {
                if !self.input.is_empty() && self.cursor > 0 {
                    let _ = self.input.remove(self.cursor - 1);
                    self.cursor = usize::min(self.cursor - 1, self.input.len());
                } else { break; }
            }
        } else {
            for _ in 0..amount {
                if !self.input.is_empty() && self.cursor < self.input.len() {
                    let _ = self.input.remove(self.cursor);
                } else { break; }
            }
        }
    }

    fn move_cursor(&mut self, offset: isize) {
        self.cursor = offset.saturating_add(self.cursor as isize).max(0).min(self.input.len() as isize) as usize;
    }

    fn to_boundary_left(&self) -> usize {
        let mut cursor = self.cursor;
        if cursor == 0 { return 0; }
        let started_space = self.input.as_bytes()[cursor - 1] == b' ';
        while cursor > 0 && ((started_space && self.input.as_bytes()[cursor - 1] == b' ') || (!started_space && self.input.as_bytes()[cursor - 1] != b' '))  {
            cursor -= 1;
            if cursor == 0 { break; }
        }
        self.cursor - cursor
    }

    fn to_boundary_right(&self) -> usize {
        let mut cursor = self.cursor;
        if cursor == self.input.len() { return 0; }
        let started_space = self.input.as_bytes()[cursor] == b' ';
        while cursor < self.input.len() && ((started_space && self.input.as_bytes()[cursor] == b' ') || (!started_space && self.input.as_bytes()[cursor] != b' '))  {
            cursor += 1;
            if cursor == self.input.len() { break; }
        }
        cursor - self.cursor
    }
}

pub(crate) fn refresh(){
    let konsole = KONSOLE.lock().unwrap();
    let settings = SETTINGS.lock().unwrap();
    clear_input_row(false);
    std::print!("\r{}{}", settings.prompt, konsole.input);
    std::print!("\r{}{}", settings.prompt, konsole.input.as_str().get(0..konsole.cursor).unwrap());
    let _ = std::io::stdout().flush();
}

pub(crate) fn clear_input_row(flush: bool){
    let (w, _) = crate::size();
    std::print!("\r{}", " ".repeat(w));
    if flush {
        let _ = std::io::stdout().flush();
    }
}