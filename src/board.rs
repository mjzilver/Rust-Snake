use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;

use crate::window;

pub const WIDTH: usize = 30;
pub const HEIGHT: usize = 30;
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const WALL_COLOR: Color = [0.00, 0.00, 0.00, 1.0];

#[derive(PartialEq)]
pub struct Board {
    pub data: Vec<Vec<Cell>>,
}

#[derive(Clone, PartialEq)]
pub enum Cell {
    Empty,
    Food,
    Snake,
    Wall,
}

impl Board {
    pub fn new() -> Board {
        let mut data = vec![vec![Cell::Empty; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if x == 0 || x == WIDTH - 1 || y == 0 || y == HEIGHT - 1 {
                    data[y][x] = Cell::Wall;
                }
            }
        }

        Board { data }
    }

    pub fn draw(&self, context: &Context, g2d: &mut G2d) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match self.data[y][x] {
                    Cell::Snake => window::draw_block(SNAKE_COLOR, x as f64, y as f64, context, g2d),
                    Cell::Food => window::draw_block(FOOD_COLOR, x as f64, y as f64, context, g2d),
                    Cell::Wall => window::draw_block(WALL_COLOR, x as f64, y as f64, context, g2d),

                    _ => {}
                };
            }
        }
    }
}
