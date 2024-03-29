use rand::{rngs::ThreadRng, Rng};

use crate::board::{self, Board, Cell};

pub struct Food {
    location: (usize, usize),
    rng: ThreadRng,
}

impl Food {
    pub fn new() -> Self {
        let rng = rand::thread_rng();

        Food {
            location: (0, 0),
            rng,
        }
    }

    pub fn update(&mut self, board: &mut Board) {
        if board.data[self.location.0][self.location.1] != Cell::Food {
            self.set_food(board)
        }
    }

    fn set_food(&mut self, board: &mut Board) {
        let rand_width = self.rng.gen_range(0..board::WIDTH);
        let rand_height = self.rng.gen_range(0..board::HEIGHT);

        if board.data[rand_height][rand_width] == Cell::Empty {
            board.data[rand_height][rand_width] = Cell::Food;
            self.location = (rand_height, rand_width)
        } else {
            self.set_food(board)
        }
    }
}
