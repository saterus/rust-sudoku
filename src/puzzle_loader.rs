use std::fs::File;
use std::io::Read;
use regex::Regex;

use Board;

static BOARD_REGEX: Regex = regex!["[^-0-9]"];

pub fn load(puzzle_path: &String) -> Board {
    println!("Loading {:?}", puzzle_path);
    let mut puzzle_file = File::open(puzzle_path).ok().expect("Could not open puzzle file!");
    let mut string_buffer = String::new();
    puzzle_file.read_to_string(&mut string_buffer).ok().expect("Error reading puzzle file!");

    let board_string = BOARD_REGEX.replace_all(&string_buffer[..], "");
    println!("Parsed board as: {:?}", board_string);

    Board::parse(board_string)
}
