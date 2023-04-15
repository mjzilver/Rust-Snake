mod food;
mod gameloop;
mod playing_board;
mod snake;
mod window;

fn main() {
    let mut gameloop = gameloop::Game::new();
    gameloop.start_loop()
}
