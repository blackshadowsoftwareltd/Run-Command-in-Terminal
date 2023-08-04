use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use console::Term;

pub fn loading_indecator() {
    print!("Loading ");

    let term = Term::stdout();
    term.hide_cursor().unwrap();

    let mut i = 0;
    loop {
        print!(".");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(500));

        i += 1;
        if i == 3 {
            i = 0;
            term.clear_chars(3).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    }
}
pub fn progress_indecator() {
    let chars = vec!["|", "/", "-", "\\"];
    let delay = Duration::from_millis(100); // Adjust the delay to change animation speed.

    std::io::stdout().flush().unwrap();
    loop {
        for c in &chars {
            print!("\rLoading {} ", c);
            std::io::stdout().flush().unwrap();
            thread::sleep(delay);
        }
    }
}
