#![allow(dead_code)]
#![allow(unused_variables)]

use std::{cell::Cell, vec};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;

#[derive(Clone)]
pub struct Board {
    pub data: Vec<Vec<(CellState, Color)>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellState {
    Active,
    Inactive,
    Empty,
}
#[derive(Clone, Debug)]
pub enum Color {
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
        let data = vec![vec![(CellState::Empty, Color::Black); WIDTH]; HEIGHT];
        Board { data }
    }

    pub fn print(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match self.data[y][x].0 {
                    CellState::Inactive => print!("🟦"),
                    CellState::Active => print!("🟥"),
                    CellState::Empty => print!("⬛"),
                };
            }
            println!("{}", y);
        }
        println!("{:?}", [0..WIDTH]);
    }

    pub fn update(&mut self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.data[y][x].0 == CellState::Active {
                    println!("{}, {} is active", y, x);
                    if y + 1 < HEIGHT {
                        if self.data[y + 1][x].0 == CellState::Empty {
                            self.data[y + 1][x].0 = CellState::Active;
                            self.data[y][x].0 = CellState::Empty;
                        } else {
                            self.data[y][x].0 = CellState::Inactive;
                        }
                    }
                }
            }
        }

        self.data[0][WIDTH / 2] = (CellState::Active, Color::Purple);
        self.data[0][WIDTH / 2 - 1] = (CellState::Active, Color::Purple);
        self.data[1][WIDTH / 2] = (CellState::Active, Color::Purple);
        self.data[1][WIDTH / 2 - 1] = (CellState::Active, Color::Purple);
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
