#![allow(dead_code)]
#![allow(unused_variables)]

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[derive(Clone)]
pub struct Board {
    pub data: Vec<Vec<Cell>>,
}

#[derive(Clone, Debug)]
pub enum Cell {
    Blue,
    Red,
    Yellow,
    Purple,
    Orange,
    Brown,
    Black,
    White,
    Green,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Board {
    pub fn init() -> Board {
        let data = vec![vec![Cell::Black; WIDTH]; HEIGHT];
        Board { data }
    }

    pub fn print(&self) {
        self.data.iter().for_each(|row| {
            row.iter().for_each(|cell| match cell {
                Cell::Blue => print!("🟦"),
                Cell::Red => print!("🟥"),
                Cell::Yellow => print!("🟨"),
                Cell::Purple => print!("🟪"),
                Cell::Orange => print!("🟧"),
                Cell::Brown => print!("🟫"),
                Cell::Black => print!("⬛"),
                Cell::White => print!("⬜"),
                Cell::Green => print!("🟩"),
            });
            println!();
        });
    }

    pub fn update(&mut self) {
        self.data[0][WIDTH / 2] = Cell::Purple;
        self.data[0][WIDTH / 2 - 1] = Cell::Purple;
        self.data[1][WIDTH / 2] = Cell::Purple;
        self.data[1][WIDTH / 2 - 1] = Cell::Purple;
    }

    pub fn handle_input(&self, dir: Direction) {
        match dir {
            Direction::Up => println!("Moving up!"),
            Direction::Down => println!("Moving down!"),
            Direction::Left => println!("Moving left!"),
            Direction::Right => println!("Moving right!"),
        }
    }
}
