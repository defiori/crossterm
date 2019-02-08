extern crate crossterm;
use self::crossterm::{Crossterm, Screen, ClearType};
use std::io::Read;
use std::{thread, time};

fn main() {
    let raw_mode = true;
    let screen = Screen::new(raw_mode);
    let crossterm: Crossterm;
    {
        let alternate_screen = screen.enable_alternate_modes(raw_mode).unwrap();
        crossterm = Crossterm::from_screen(&alternate_screen.screen);
    }
    // <-- Dropping alternate_screen here

    let mut input = crossterm.input().read_async().bytes();
    let terminal = crossterm.terminal();
    let cursor = crossterm.cursor();
    cursor.hide().unwrap();

    loop {
        terminal.clear(ClearType::All).unwrap();
        cursor.goto(1, 1).unwrap();
        match input.next() {
            // These user inputs are not in raw mode
            Some(ipt) => {
                terminal.write(format!("{:?}    <- Character pressed", ipt)).unwrap();
                match ipt {
                    // Does not return to original screen
                    Ok(b'q') => break,
                    _ => {},
                };
            },
            _ => {}
        };
        thread::sleep(time::Duration::from_millis(200));
    }
}