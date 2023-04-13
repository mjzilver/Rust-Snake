use rand::{rngs::ThreadRng, Rng};

use crate::playing_board::{self, Board, Cell};

#[derive(Debug)]
pub struct Food {
    location: (usize, usize),
    rng: ThreadRng,
}

impl Food {
    pub fn new(board: &mut Board) -> Self {
        let mut rng = rand::thread_rng();
        let rand_width = rng.gen_range(0..playing_board::WIDTH);
        let rand_height = rng.gen_range(0..playing_board::HEIGHT);

        board.data[rand_height][rand_width] = Cell::Food;

        return Food {
            location: (rand_height, rand_width),
            rng,
        };
    }

    pub fn update(&mut self, board: &mut Board) {
        if board.data[self.location.0][self.location.1] != Cell::Food {
            self.set_food(board)
        }
    }

    fn set_food(&mut self, board: &mut Board) {
        let rand_width = self.rng.gen_range(0..playing_board::WIDTH);
        let rand_height = self.rng.gen_range(0..playing_board::HEIGHT);

        if board.data[rand_height][rand_width] == Cell::Empty {
            board.data[rand_height][rand_width] = Cell::Food;
            self.location = (rand_height, rand_width)
        } else {
            self.set_food(board)
        }
    }
}
