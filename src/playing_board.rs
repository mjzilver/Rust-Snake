#![allow(dead_code)]
#![allow(unused_variables)]

const WIDTH: usize = 30;
const HEIGHT: usize = 10;

pub struct Board {
    pub data: Vec<Vec<Cell>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Food,
    Snake,
}

impl Board {
    pub fn init() -> Board {
        let data = vec![vec![Cell::Empty; WIDTH]; HEIGHT];

        return Board { data };
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
    }
}
