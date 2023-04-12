use crate::playing_board::{self, Board, Cell};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}
#[derive(Debug)]
pub struct Snake {
    body: Vec<(usize, usize)>,
    direction: Direction,
    digesting: bool,
}

impl Snake {
    pub fn new(start: (usize, usize), direction: Direction) -> Self {
        let mut body: Vec<(usize, usize)> = vec![];
        body.push(start);

        return Self {
            body,
            direction,
            digesting: false,
        };
    }

    pub fn update(&mut self, board: &mut Board) {
        for i in (0..self.body.len()).rev() {
            let mut new_coord = &mut self.body[i];

            board.data[new_coord.0][new_coord.1] = Cell::Empty;

            match self.direction {
                Direction::Up => new_coord.0 -= 1,
                Direction::Down => new_coord.0 += 1,
                Direction::Left => new_coord.1 -= 1,
                Direction::Right => new_coord.1 += 1,
                Direction::None => {}
            }

            board.data[new_coord.0][new_coord.1] = Cell::Snake;
        }
    }

    pub fn movement(&mut self, dir: Direction) {
        self.direction = dir;
    }
}
