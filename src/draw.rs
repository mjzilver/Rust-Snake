use crate::playing_board::Cell;
use crate::playing_board::{self, Board};

pub fn print(playing_board: Board) {
    for y in 0..playing_board::HEIGHT {
        for x in 0..playing_board::WIDTH {
            match playing_board.data[y][x] {
                Cell::Snake => print!("🐍"),
                Cell::Food => print!("🍎"),
                Cell::Empty => print!("⬛"),
            };
        }
        println!("{}", y);
    }
}
