use crate::food::Food;
use crate::playing_board::{self, Board};
use crate::snake::Direction;
use crate::{snake, window};
use piston_window::types::Color;
use piston_window::*;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const MOVING_PERIOD: f64 = 0.1;

pub fn start_loop() {
    let mut window: PistonWindow = WindowSettings::new(
        "Snake",
        (
            (playing_board::WIDTH as f64) * window::BLOCK_SIZE,
            (playing_board::HEIGHT as f64) * window::BLOCK_SIZE,
        ),
    )
    .exit_on_esc(true)
    .automatic_close(true)
    .build()
    .unwrap();

    // Initialize board instance
    let mut playing_board: Board = playing_board::Board::init();
    let mut snake = snake::Snake::new((5, 5), Direction::Down);
    let mut food: Food = Food::new(&mut playing_board);
    let mut waiting_time: f64 = 0.0;

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::W => snake.movement(Direction::Up),
                Key::A => snake.movement(Direction::Left),
                Key::S => snake.movement(Direction::Down),
                Key::D => snake.movement(Direction::Right),
                _ => {}
            }
        }
        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            playing_board.draw(&c, g);
        });
        event.update(|arg| {
            waiting_time += arg.dt;

            if waiting_time > MOVING_PERIOD {
                snake.update(&mut playing_board);
                food.update(&mut playing_board);
                waiting_time = 0.0;
            }
        });
    }
}
