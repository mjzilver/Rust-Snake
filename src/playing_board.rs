#![allow(dead_code)]
#![allow(unused_variables)]

const WIDTH: usize = 30;
const HEIGHT: usize = 10;

#[derive(Clone)]
pub struct Board<'a> {
    pub data: Vec<Vec<Cell>>,
    pub dir: Direction,
    pub snake: &'a Cell,
    pub score: i16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Food,
    Snake,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Board<'_> {
    pub fn init() -> &'static Board<'static> {
        let mut data = vec![vec![Cell::Empty; WIDTH]; HEIGHT];
        data[5][5] = Cell::Snake;
        data[5][10] = Cell::Food;

        let board = Board {
            data: data,
            dir: Direction::None,
            snake: &data[5][5],
            score: 0,
        };

        return &board;
    }

    pub fn print(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match self.data[y][x] {
                    Cell::Snake => print!("🐍"),
                    Cell::Food => print!("🍎"),
                    Cell::Empty => print!("⬛"),
                };
            }
            println!("{}", y);
        }
        println!("{:?}", 0..WIDTH);
    }

    pub fn handle_input(&mut self, dir: Direction) {
        self.dir = dir;

        match self.dir {
            Direction::Up => {}
            Direction::Down => {}
            Direction::Left => {}
            Direction::Right => {}
            Direction::None => {}
        }
    }
}
