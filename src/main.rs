mod position;
mod terminal;
mod torch;

use position::Position;
use terminal::{get_raw_stdout, handle_events};
use torch::update_screen;


fn main() {
    let mut position = Position::new(2, 3);
    let mut stdout = get_raw_stdout().unwrap();
    let file_contents = std::fs::read_to_string("input.txt").unwrap();
    let contents: Vec<&str> = file_contents.lines().collect();

    update_screen(&mut stdout, &contents, &position).unwrap();

    handle_events(&mut |key| {
        position.move_with_key(key);
        update_screen(&mut stdout, &contents, &position).unwrap();
    });
}

