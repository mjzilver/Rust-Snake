use piston_window::types::Color;
use piston_window::Context;
use piston_window::G2d;

pub const WIDTH: usize = 30;
pub const HEIGHT: usize = 30;
const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];
const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];

use crate::window::draw_block;

#[derive(Clone, Debug, PartialEq, Eq)]
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

    pub fn draw(&self, context: &Context, g2d: &mut G2d) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                match self.data[y][x] {
                    Cell::Snake => draw_block(SNAKE_COLOR, x as f64, y as f64, context, g2d),
                    Cell::Food => draw_block(FOOD_COLOR, x as f64, y as f64, context, g2d),
                    _ => {}
                };
            }
        }
    }
}
