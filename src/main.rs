use std::io::{stdin, stdout};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

mod position;
mod terminal;

use position::Position;
use terminal::{clear_screen, draw_screen, move_cursor};

fn handle_key(position: &mut Position, key: char) {
    match key {
        'j' => position.go_down(),
        'k' => position.go_up(),
        'h' => position.go_left(),
        'l' => position.go_right(),
        _ => (),
    }
}

fn euclidian_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).pow(2) + (y1 - y2).pow(2)
}

fn filter_text_around_position(lines: &[String], position: &Position) -> Vec<String> {
    let max_distance = 10;
    let position_x: i32 = position.x().into();
    let position_y: i32 = position.y().into();
    lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, s)| {
                    let distance_to_cursor = euclidian_distance(
                        x.try_into().unwrap(),
                        y.try_into().unwrap(),
                        position_x - 1,
                        position_y - 1,
                    );
                    if distance_to_cursor < max_distance {
                        s
                    } else {
                        ' '
                    }
                })
                .collect()
        })
        .collect()
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    clear_screen(&mut stdout).unwrap();

    let mut position = Position::new(2, 3);
    let contents = vec!["hello".to_string(), "world".to_string()];
    let filtered_contents = filter_text_around_position(&contents, &position);
    draw_screen(&mut stdout, &filtered_contents.iter().map(|s| &s[..]).collect()).unwrap();
    move_cursor(&mut stdout, &position).unwrap();

    for c in stdin.events() {
        if let Event::Key(key) = c.unwrap() {
            match key {
                Key::Ctrl('c') => break,
                Key::Char(key) => {
                    handle_key(&mut position, key);
                    let filtered_contents = filter_text_around_position(&contents, &position);
                    draw_screen(&mut stdout, &filtered_contents.iter().map(|s| &s[..]).collect()).unwrap();
                    move_cursor(&mut stdout, &position).unwrap();
                }
                _ => {}
            }
        }
    }
}
