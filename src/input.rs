//  input.rs
use crate::game::{Direction, Game, move_piece};
use crossterm::event::{Event, KeyCode, poll, read};
use std::time::Duration;

//  Input
pub fn handle_input(game: &mut Game) -> std::io::Result<()> {
    if poll(Duration::ZERO)? {
        let event = read()?;

        match event {
            Event::Key(key) => match key.code {
                KeyCode::Left => {
                    move_piece(game, Direction::Left);
                }
                KeyCode::Right => {
                    move_piece(game, Direction::Right);
                }
                KeyCode::Esc => game.game_over = true,
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}
