mod playing_board {
    const WIDTH: u16 = 10;
    const HEIGHT: u16 = 20;

    #[derive(Clone)]
    pub struct Board<'a> {
        pub data: Vec<Vec<Cell>>,
        pub current: Vec<&'a Cell>,
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

    impl Board<'_> {
        pub fn init() -> Board<'static> {
            let data = vec![vec![Cell::Black; WIDTH as usize]; HEIGHT as usize];
            let current = vec![];
            Board { data, current }
        }

        pub fn print(&self) {
            for row in &self.data {
                for cell in row {
                    match cell {
                        Cell::Blue => print!("🟦"),
                        Cell::Red => print!("🟥"),
                        Cell::Yellow => print!("🟨"),
                        Cell::Purple => print!("🟪"),
                        Cell::Orange => print!("🟧"),
                        Cell::Brown => print!("🟫"),
                        Cell::Black => print!("⬛"),
                        Cell::White => print!("⬜"),
                        Cell::Green => print!("🟩"),
                    }
                }
                println!()
            }
        }
    }
}

mod gameloop {
    use crate::playing_board;

    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    pub fn handle_input(dir: Direction) {
        match dir {
            Direction::Up => println!("Moving up!"),
            Direction::Down => println!("Moving down!"),
            Direction::Left => println!("Moving left!"),
            Direction::Right => println!("Moving right!"),
        }
    }

    pub fn start_loop() {
        let mut game_is_running = true;

        // Initialize board instance
        let mut playing_board = playing_board::Board::init();

        while game_is_running {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            match input.chars().next().unwrap() {
                'x' => game_is_running = false,
                'w' => handle_input(Direction::Up),
                'a' => handle_input(Direction::Left),
                's' => handle_input(Direction::Down),
                'd' => handle_input(Direction::Right),
                _ => println!("You entered a wrong value!"),
            }

            playing_board.print();
        }
    }
}

pub fn main() {
    gameloop::start_loop()
}
