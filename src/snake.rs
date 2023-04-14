use crate::playing_board::{self, Board, Cell};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Debug)]
pub struct Snake {
    body: Vec<(usize, usize)>,
    direction: Direction,
    digesting: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnakeStatus {
    Collision,
    Moving,
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

    pub fn update(&mut self, board: &mut Board) -> SnakeStatus {
        let mut old_coord = self.body.last().copied().unwrap();
        let mut new_coord = &mut self.body.pop().unwrap();

        match self.direction {
            Direction::Up => {
                if new_coord.0 == 0 {
                    new_coord.0 = playing_board::HEIGHT
                }
                new_coord.0 -= 1
            }
            Direction::Down => new_coord.0 += 1,
            Direction::Left => {
                if new_coord.1 == 0 {
                    new_coord.1 = playing_board::WIDTH
                }
                new_coord.1 -= 1
            }
            Direction::Right => new_coord.1 += 1,
        }

        new_coord.0 %= playing_board::HEIGHT;
        new_coord.1 %= playing_board::WIDTH;

        if board.data[new_coord.0][new_coord.1] == Cell::Food {
            self.digesting = true;
        } else if board.data[new_coord.0][new_coord.1] == Cell::Snake {
            return SnakeStatus::Collision;
        }

        if !self.digesting {
            board.data[old_coord.0][old_coord.1] = Cell::Empty;
        } else {
            self.body.push(old_coord);
            self.digesting = false;
        }

        board.data[new_coord.0][new_coord.1] = Cell::Snake;
        self.body.push(*new_coord);

        for i in (0..self.body.len() - 1).rev() {
            board.data[old_coord.0][old_coord.1] = Cell::Snake;
            board.data[self.body[i].0][self.body[i].1] = Cell::Empty;
            (self.body[i], old_coord) = (old_coord, self.body[i]);
        }
        return SnakeStatus::Moving;
    }

    pub fn movement(&mut self, dir: Direction) {
        match dir {
            Direction::Up if self.direction != Direction::Down => self.direction = dir,
            Direction::Down if self.direction != Direction::Up => self.direction = dir,
            Direction::Left if self.direction != Direction::Right => self.direction = dir,
            Direction::Right if self.direction != Direction::Left => self.direction = dir,
            _ => {}
        }
    }
}
