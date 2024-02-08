use std::io::{Stdout, Error};

use crate::terminal::{move_cursor, draw_screen};
use crate::position::Position;

fn euclidian_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    (x1 - x2).pow(2) + (y1 - y2).pow(2)
}

pub fn filter_text_around_position(lines: &[&str], position: &Position) -> Vec<String> {
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

pub fn update_screen(stdout: &mut Stdout, contents: &[&str], position: &Position) -> Result<(), Error> {
    let filtered_contents = filter_text_around_position(contents, position);
    draw_screen(
        stdout,
        &filtered_contents.iter().map(|s| &s[..]).collect(),
        )?;
    move_cursor(stdout, position)?;
    Ok(())
}
