use rand::Rng;

use crate::playing_board::{self, Board, Cell};

#[derive(Debug)]
pub struct Food {
    location: Option<(usize, usize)>,
}

impl Food {
    pub fn new(board: &mut Board) -> Self {
        let mut rng = rand::thread_rng();
        let rand_width = rng.gen_range(0..playing_board::WIDTH);
        let rand_height = rng.gen_range(0..playing_board::HEIGHT);

        board.data[rand_height][rand_width] = Cell::Food;

        return Food {
            location: Some((rand_height, rand_width)),
        };
    }
}
