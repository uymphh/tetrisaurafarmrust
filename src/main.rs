//main.rs
mod game;
mod input;
mod pieces;
mod render;

use crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::stdout;
use std::time::{Duration, Instant};

use game::Game;
use input::{InputState, handle_input};
use render::render_game;

//Main
fn main() -> std::io::Result<()> {
    let mut input = InputState {
        held_left: None,
        held_right: None,
        last_left_repeat: None,
        last_right_repeat: None,
        last_horizontal: None,
    };
    enable_raw_mode()?;
    execute!(
        stdout(),
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::REPORT_EVENT_TYPES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
        )
    )?;
    let mut game = Game::new();
    let mut last_tick = Instant::now();
    while !game.is_game_over() {
        handle_input(&mut game, &mut input)?;
        input.update(&mut game);
        if last_tick.elapsed() >= Duration::from_secs_f64(0.2) {
            game.update();
            render_game(&game);
            last_tick = Instant::now();
            println!("Score: {}", game.score());
            std::thread::sleep(Duration::from_millis(1));
        }
    }
    execute!(stdout(), PopKeyboardEnhancementFlags)?;
    disable_raw_mode()?;
    println!("Game Over");
    Ok(())
}
