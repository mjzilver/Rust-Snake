use crate::playing_board;
use crate::playing_board::Cell;
use crate::snake;
use crate::snake::Direction;

pub fn start_loop() {
    let mut game_is_running = true;

    // Initialize board instance
    let mut playing_board = playing_board::Board::init();
    let mut start: &mut Cell = &mut playing_board.data[5][5];
    let mut snake = snake::Snake::new(start, Direction::None);

    while game_is_running {
        playing_board.print();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.chars().next().unwrap() {
            'x' => game_is_running = false,
            'w' => {}
            'a' => {}
            's' => {}
            'd' => {}
            _ => println!("You entered a wrong value!"),
        }
    }
}
