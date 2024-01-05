use std::time::Duration;

#[macro_use]
extern crate konsole;

fn main() {
    konsole::set_prompt(">");
    konsole::set_history_enabled(true);
    konsole::activate();
    println!("hello, world from konsole!");
    std::thread::spawn(|| {
        let mut i = 0;
        while konsole::is_active() {
            println!("counting {}", i);
            std::thread::sleep(Duration::from_secs(2));
            i += 1;
            //if i == 5 {
            //    konsole::deactivate(Some("Press any key to exit konsole..."));
            //}
        }
    });
    while konsole::is_active() {
        for input in konsole::queued_inputs().into_iter().filter(|i| !i.is_empty()) {
            println!("inputted: {input:?}");
        }
    }
}