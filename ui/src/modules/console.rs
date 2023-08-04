use std::{thread, time::Duration};

use console::{style, Term};

pub fn play_with_console_crates() {
    let term = Term::stdout();
    term.set_title("Counting...");
    term.write_line("Going to do some counting now").unwrap();
    term.hide_cursor().unwrap();
    for x in 0..10 {
        if x != 0 {
            term.move_cursor_up(1).unwrap();
        }
        term.write_line(&format!("Counting {}/10", style(x + 1).cyan()))
            .unwrap();
        thread::sleep(Duration::from_millis(400));
    }
    term.show_cursor().unwrap();
    term.clear_last_lines(1).unwrap();
    term.write_line("Done counting!").unwrap();
}
