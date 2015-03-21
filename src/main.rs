extern crate sudoku;

use sudoku::Board;
use sudoku::Square;

fn main() {
    let known = Square::known(4);
    let guess = Square::new();
    println!("known: {:?}, guess: {:?}", known, guess);

    let board = Board::new();
    println!("board:\n{:?}", board);
}
