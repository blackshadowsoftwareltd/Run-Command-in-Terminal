use console::Term;
use dialoguer::{theme::ColorfulTheme, Input};

use std::thread;
use std::time::Duration;

pub fn buffere() {
    let term = Term::buffered_stderr();

    loop {
        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Your name")
            .interact_on(&term)
            .unwrap();
        println!("You hame entered: {}", name);
        clear_terminal();
    }
}

fn clear_terminal() {
    let term = Term::stdout();
    term.write_line("Clear will be 2 seconds").unwrap();
    thread::sleep(Duration::from_millis(2000));
    // term.clear_last_lines(1).unwrap();
    term.clear_screen().unwrap();
}
