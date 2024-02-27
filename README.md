# Konsole - Threaded Interaction
Jealous of the minecraft console, with its ability to print and receive input simultaneously?
Already warily side-eyeing ui alternatives? All your problems end today - with konsole!

Konsole supports both [windows](src/windows.rs) and [unix](src/unix.rs), featuring a custom implementation 
of many of the keyboard featurs the normal system `input` has, such as shift-delete.
`Tab` behavior is configurable with a custom [autocompletion](examples/example.rs).

This crate provides its own `print!()` and `println!()` macros, requiring downstream crates to use it instead of `std::print!()` and `std::println!()`.
Alternatively, you can use `konsole::print::printout(line: impl AsRef<str>)` to override your logger's output. Printing using methods outside of what
this crate provides will result in unwanted behavior."

# Basic Usage
[example](examples/example.rs)
```rs
use std::time::Duration;

// to override print/println crate-wide
#[macro_use]
extern crate konsole;

fn main() {
    konsole::edit_settings(|settings| {
        settings.history_enabled = true;
    }
    
    konsole::activate();
    println!("hello, world from konsole!");

    // some pretend-logs
    std::thread::spawn(|| {
        let mut i = 0;
        while konsole::is_active() {
            println!("counting {}", i);
            std::thread::sleep(Duration::from_secs(2));
            i += 1;
        }
    });

    // handle input
    while konsole::is_active() {
        for input in konsole::queued_inputs().into_iter() {
            println!(">{input}");
            let input = input.trim();
            if input.is_empty() {
                println!();
            } else {
                if input == "help" {
                    println!("[generic help message]");
                } else {
                    println!("received: {input:?}");
                }
            }
        }
    }
}
```