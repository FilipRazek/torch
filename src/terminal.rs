use std::io::stdin;
use std::io::stdout;
use std::io::Error;
use std::io::{Stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;

use crate::position::Position;

pub fn move_cursor(stdout: &mut Stdout, position: &Position) -> Result<(), Error> {
    write!(
        stdout,
        "{}",
        termion::cursor::Goto(position.x(), position.y())
    )?;
    stdout.flush()?;
    Ok(())
}

fn clear_screen(stdout: &mut Stdout) -> Result<(), Error> {
    write!(stdout, "{}", termion::clear::All)?;
    stdout.flush()?;
    Ok(())
}

pub fn draw_screen(stdout: &mut Stdout, lines: &Vec<&str>) -> Result<(), Error> {
    clear_screen(stdout)?;
    let top_left = Position::new(1, 1);
    move_cursor(stdout, &top_left)?;
    for line in lines {
        write!(stdout, "{}\r\n", line)?;
    }
    stdout.flush()?;
    Ok(())
}

pub fn handle_events(handler: &mut impl FnMut(char)) {
    let stdin = stdin();
    for c in stdin.events() {
        if let Event::Key(key) = c.unwrap() {
            match key {
                Key::Ctrl('c') => break,
                Key::Char(key) => {
                    handler(key);
                }
                _ => {}
            }
        }
    }
}

pub fn get_raw_stdout() -> Result<RawTerminal<Stdout>, Error> {
    stdout().into_raw_mode()
}

pub fn get_terminal_middle() -> Result<(u16, u16), Error> {
    let (x, y) = termion::terminal_size()?;
    Ok((x / 2, y / 2))
}
