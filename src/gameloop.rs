use crate::food::Food;
use crate::playing_board::{self, Board};
use crate::snake;
use crate::snake::Direction;

pub fn start_loop() {
    let mut game_is_running = true;

    // Initialize board instance
    let mut playing_board: Board = playing_board::Board::init();
    let mut snake = snake::Snake::new((5, 5), Direction::None);
    let mut food: Food = Food::new(&mut playing_board);

    while game_is_running {
        snake.update(&mut playing_board);

        playing_board.print();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.chars().next().unwrap() {
            'x' => game_is_running = false,
            'w' => snake.movement(Direction::Up),
            'a' => snake.movement(Direction::Left),
            's' => snake.movement(Direction::Down),
            'd' => snake.movement(Direction::Right),
            _ => println!("You entered a wrong value!"),
        }
    }
}
