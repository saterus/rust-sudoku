extern crate sudoku;

use sudoku::Board;
use sudoku::Square;

fn main() {
    let known = Square::known(4);
    let guess = Square::new();
    println!("known: {:?}, guess: {:?}", known, guess);

    let board = Board::new();
    println!("board:\n{:?}", board);

    let b1_str = "--1------5------9-----------------------------7----------------------------------".to_string();;
    let b1 = Board::parse(b1_str);
    println!("b1:\n{:?}", b1);
}
