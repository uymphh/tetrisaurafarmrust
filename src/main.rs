//main.rs
mod game;
mod input;
mod pieces;
mod render;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::time::{Duration, Instant};

use game::Game;
use input::handle_input;
use render::render_game;

//Main
fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut game = Game::new();
    let mut last_tick = Instant::now();
    while !game.is_game_over() {
        handle_input(&mut game)?;
        if last_tick.elapsed() >= Duration::from_secs_f64(0.2) {
            game.update();
            render_game(&game);
            last_tick = Instant::now();
            println!("Score: {}", game.score());
        }
    }
    disable_raw_mode()?;
    println!("Game Over");
    Ok(())
}
