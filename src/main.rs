use std::io::Error;

use std::io::{stdin, stdout, Stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

mod position;
use position::Position;

fn move_cursor(stdout: &mut RawTerminal<Stdout>, position: &Position) -> Result<(), Error> {
    write!(stdout, "{}", termion::cursor::Goto(position.x(), position.y()))?;
    Ok(())
}

fn handle_key(position: &mut Position, key: char) {
    match key {
        'j' => position.go_down(),
        'k' => position.go_up(),
        'h' => position.go_left(),
        'l' => position.go_right(),
        _ => (),
    }
}

fn clear_screen(stdout: &mut Stdout) {
    write!(stdout, "{}", termion::clear::All).unwrap();
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    clear_screen(&mut stdout);

    let mut position = Position::new(1, 1);
    move_cursor(&mut stdout, &position).unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        if let Event::Key(key) = c.unwrap() {
            match key {
                Key::Ctrl('c') => break,
                Key::Char(key) => {
                    eprint!("{}", key);
                    handle_key(&mut position, key);
                    let _ = move_cursor(&mut stdout, &position);
                }
                _ => {}
            }
        }
        stdout.flush().unwrap();
    }
}
