use crate::playing_board::{Board, Cell};

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
        body.push((start.0 + 1, start.1));
        body.push((start.0 + 2, start.1));

        return Self {
            body,
            direction,
            digesting: false,
        };
    }

    pub fn update(&mut self, board: &mut Board) {
        for i in (0..self.body.len()).rev() {
            let old_coord = self.body[i].clone();
            let mut new_coord = &mut self.body.remove(i);

            match self.direction {
                Direction::Up => new_coord.0 -= 1,
                Direction::Down => new_coord.0 += 1,
                Direction::Left => new_coord.1 -= 1,
                Direction::Right => new_coord.1 += 1,
                Direction::None => {}
            }

            if board.data[new_coord.0][new_coord.1] == Cell::Food {
                self.digesting = true;
            }

            if !self.digesting {
                board.data[old_coord.0][old_coord.1] = Cell::Empty;
            } else {
                self.body.push(old_coord);
                self.digesting = false;
            }

            board.data[new_coord.0][new_coord.1] = Cell::Snake;
            self.body.push(*new_coord);
        }
    }

    pub fn movement(&mut self, dir: Direction) {
        self.direction = dir;
    }
}
