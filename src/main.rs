extern crate sudoku;

use std::env;

use sudoku::puzzle_loader;
use sudoku::each_slice::EachSlice;

#[allow(dead_code)]
fn main() {

    let v = vec![1i32,2,3,4,5,6,7,8,9,10];
    for s in v.iter().each_slice(3) {
        println!("s = {:?}", s);
    }


    let puzzle_path = env::args().nth(1).expect("Must pass a path to a puzzle file as first argument!");
    let board = puzzle_loader::load(&puzzle_path);
    println!("board size: {:?}", board.total_squares());
    println!("\n\n{:?}", board);
}
