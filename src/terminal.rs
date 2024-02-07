use std::io::Error;
use std::io::{Stdout, Write};

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

pub fn clear_screen(stdout: &mut Stdout) -> Result<(), Error> {
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
