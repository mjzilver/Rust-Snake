use crate::food::Food;
use crate::board::{self, Board};
use crate::snake::{Snake, Direction, SnakeStatus};
use crate::{snake, window};
use piston_window::types::Color;
use piston_window::*;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
const MOVING_PERIOD: f64 = 0.1;
const SNAKE_START: (usize, usize) = (5, 5);
const SCREEN_WIDTH: f64 = (board::WIDTH as f64) * window::BLOCK_SIZE;
const SCREEN_HEIGHT: f64 = (board::HEIGHT as f64) * window::BLOCK_SIZE;

pub struct Game {
    board: Board,
    snake: Snake,
    food: Food,
    waiting_time: f64
}

impl Game {
    pub fn new() -> Game {
        return Game { board: board::Board::new(), 
            snake: snake::Snake::new(SNAKE_START, Direction::Down), 
            food: Food::new(), 
            waiting_time: 0.0 }
    }

    pub fn start_loop(&mut self) {
        let mut window: PistonWindow = WindowSettings::new(
            "Snake",
            ( SCREEN_WIDTH, SCREEN_HEIGHT ),
        )
        .exit_on_esc(true)
        .automatic_close(true)
        .build()
        .expect("Window failed to load");
    
        while let Some(event) = window.next() {
            if let Some(Button::Keyboard(key)) = event.release_args() {
                self.input(&key)
            }
        
            window.draw_2d(&event, |context, g2d, _| {
                clear(BACK_COLOR, g2d);
                self.board.draw(&context, g2d);
                if self.snake.status == SnakeStatus::Collision {
                    window::draw_rect([0.8, 0.0, 0.0, 0.5], 0.0, 0.0, SCREEN_WIDTH, SCREEN_HEIGHT , &context, g2d)
                }
            });
            event.update(|arg| {
                self.update(arg)
            });
        }
    }

    fn input (&mut self, key: &Key) {
        match key {
            Key::W => self.snake.movement(Direction::Up),
            Key::A => self.snake.movement(Direction::Left),
            Key::S => self.snake.movement(Direction::Down),
            Key::D => self.snake.movement(Direction::Right),
            Key::R => {*self = Game::new()},
            _ => {}
        }
    }

    fn update(&mut self, arg: &UpdateArgs) {
        self.waiting_time += arg.dt;
    
        if self.waiting_time > MOVING_PERIOD {
            if self.snake.status == SnakeStatus::Moving {
                self.snake.update(&mut self.board);
                self.food.update(&mut self.board);
            } 
            self.waiting_time = 0.0;
        }
    }
}


