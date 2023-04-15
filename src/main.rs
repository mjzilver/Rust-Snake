mod food;
mod game;
mod board;
mod snake;
mod window;

fn main() {
    let mut game = game::Game::new();
    game.start_loop()
}
