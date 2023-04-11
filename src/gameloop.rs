use crate::playing_board;

pub fn start_loop() {
    let mut game_is_running = true;

    // Initialize board instance
    let mut playing_board = playing_board::Board::init();

    while game_is_running {
        playing_board.print();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.chars().next().unwrap() {
            'x' => game_is_running = false,
            'w' => playing_board.handle_input(playing_board::Direction::Up),
            'a' => playing_board.handle_input(playing_board::Direction::Left),
            's' => playing_board.handle_input(playing_board::Direction::Down),
            'd' => playing_board.handle_input(playing_board::Direction::Right),
            _ => println!("You entered a wrong value!"),
        }

        playing_board.update();
    }
}
