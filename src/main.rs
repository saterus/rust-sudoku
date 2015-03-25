extern crate sudoku;

use std::env;
use std::path::Path;

use sudoku::Board;
use sudoku::Square;

use sudoku::puzzle_loader;

fn main() {
    let puzzle_path = env::args().nth(1).expect("Must pass a path to a puzzle file as first argument!");
    let board = puzzle_loader::load(&puzzle_path);
    println!("board size: {:?}", board.total_squares());
    println!("\n\n{:?}", board);
}
