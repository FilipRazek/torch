mod position;
mod terminal;
mod torch;
mod arg_parser;

use position::Position;
use terminal::{get_raw_stdout, handle_events, get_terminal_middle};
use torch::update_screen;
use arg_parser::get_application_args;


fn main() {
    let (filename,) = get_application_args().unwrap();
    let (x, y) = get_terminal_middle().unwrap();
    let mut position = Position::new(x, y);
    let mut stdout = get_raw_stdout().unwrap();
    let file_contents = std::fs::read_to_string(filename).unwrap();
    let contents: Vec<&str> = file_contents.lines().collect();

    update_screen(&mut stdout, &contents, &position).unwrap();

    handle_events(&mut |key| {
        position.move_with_key(key);
        update_screen(&mut stdout, &contents, &position).unwrap();
    });
}

