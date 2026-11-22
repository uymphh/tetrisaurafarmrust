//  input.rs
use crate::game::{Direction, Game, Rotation};
use crossterm::event::{Event, KeyCode, poll, read};
use std::time::Duration;

//  Input
pub fn handle_input(game: &mut Game) -> std::io::Result<()> {
    if poll(Duration::ZERO)? {
        let event = read()?;

        match event {
            Event::Key(key) => match key.code {
                KeyCode::Left => {
                    game.move_piece(Direction::Left);
                }
                KeyCode::Right => {
                    game.move_piece(Direction::Right);
                }
                KeyCode::Down => {
                    game.move_piece(Direction::Down);
                }
                KeyCode::Char('x') => {
                    game.rotate_piece(Rotation::Clockwise);
                }
                KeyCode::Char('z') => {
                    game.rotate_piece(Rotation::CounterClockwise);
                }
                KeyCode::Char('c') => {
                    game.hold();
                }
                KeyCode::Up => {
                    game.hard_drop();
                }
                KeyCode::Esc => game.end(),
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}
