use std::time::Duration;
use std::sync::Arc;

// to enable print/println crate-wide
#[macro_use]
extern crate konsole;

fn main() {
    konsole::edit_settings(|settings| {
        settings.history_enabled = true;

        settings.tab_complete = Arc::new(|query| {
            let (left, right) = query.input.split_at(query.cursor_before);
            if let Some(word) = left.split(' ').last() {
                if !word.is_empty() {
                    let COMPLETES = ["rust", "rustacean", "rusty", "rustic"];
                    let choices = COMPLETES.into_iter().filter(|c| c.starts_with(word)).collect::<Vec<_>>();
                    if !choices.is_empty() {
                        let mut index = query.tab_repeat % (choices.len() + 1);
                        if choices.len() == 1 {
                            index = 1;
                        }
                        return Some(if index == 0 {
                            if choices.len() != 1 && query.tab_repeat == 0 {
                                println!("? {}", choices.join(", "));
                            }
                            let right = right.split_at(query.cursor_position - query.cursor_before).1;
                            konsole::TabResult {
                                output: left.to_string() + right,
                                cursor_movement: query.cursor_before as isize - query.cursor_position as isize
                            }
                        } else {
                            let len = word.len();
                            let complete = choices[index - 1];
                            let left = left.split_at(query.cursor_before - word.len()).0.to_string();
                            let right = right.split_at(query.cursor_position - query.cursor_before).1;
                            konsole::TabResult {
                                output: left + complete + right,
                                cursor_movement: query.cursor_before as isize - query.cursor_position as isize + complete.len() as isize - len as isize
                            }
                        })
                    }
                }
            }
            Some(konsole::TabResult {
                output: query.input,
                cursor_movement: 0
            })
        });
    });

    konsole::activate();
    println!("hello, world from konsole!");

    std::thread::spawn(|| {
        let mut i = 0;
        while konsole::is_active() {
            //println!("counting {}", i);
            std::thread::sleep(Duration::from_secs(2));
            i += 1;
            //if i == 5 {
            //    konsole::deactivate(Some("Press any key to exit konsole..."));
            //}
        }
    });

    while konsole::is_active() {
        for input in konsole::queued_inputs().into_iter() {
            println!(">{input}");
            let input = input.trim();
            if input.is_empty() {
                println!();
            } else {
                println!("received: {input:?}");
            }
        }
    }
}