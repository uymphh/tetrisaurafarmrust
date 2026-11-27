//  input.rs
use crate::game::{Direction, Game, Rotation};
use crossterm::event::ModifierKeyCode;
use crossterm::event::{Event, KeyCode, KeyEventKind, poll, read};
use std::time::Duration;
use std::time::Instant;

// Types
pub struct InputState {
    pub held_right: Option<Instant>,
    pub held_left: Option<Instant>,
    pub last_left_repeat: Option<Instant>,
    pub last_right_repeat: Option<Instant>,
    pub last_horizontal: Option<Direction>,
}
impl InputState {
    pub fn last(&mut self, direction: Option<Direction>) {
        self.last_horizontal = direction;
    }
    pub fn update(&mut self, game: &mut Game) {
        match self.last_horizontal {
            Some(Direction::Left) => Self::update_direction(
                &self.held_left,
                &mut self.last_left_repeat,
                Direction::Left,
                game,
            ),
            Some(Direction::Right) => Self::update_direction(
                &self.held_right,
                &mut self.last_right_repeat,
                Direction::Right,
                game,
            ),
            _ => {}
        }
    }
    fn update_direction(
        held: &Option<Instant>,
        last_repeat: &mut Option<Instant>,
        direction: Direction,
        game: &mut Game,
    ) {
        if let Some(held) = held {
            if held.elapsed() >= Duration::from_millis(170) {
                if let Some(last_held) = last_repeat {
                    if last_held.elapsed() >= Duration::from_millis(30) {
                        game.move_piece(direction);
                        *last_repeat = Some(Instant::now());
                    }
                } else {
                    game.move_piece(direction);
                    *last_repeat = Some(Instant::now());
                }
            }
        }
        // DAS / ARR logic goes here
    }
}

//  Input
pub fn handle_input(game: &mut Game, input: &mut InputState) -> std::io::Result<()> {
    while poll(Duration::ZERO)? {
        let event = read()?;

        match event {
            Event::Key(key) => match key.code {
                KeyCode::Left => match key.kind {
                    KeyEventKind::Press => {
                        if input.held_left.is_none() {
                            input.held_left = Some(Instant::now());
                            game.move_piece(Direction::Left);
                            input.last(Some(Direction::Left));
                        }
                    }
                    KeyEventKind::Release => {
                        input.last_left_repeat = None;
                        input.held_left = None;
                        if input.last_horizontal == Some(Direction::Left) {
                            if input.held_right.is_some() {
                                input.last(Some(Direction::Right));
                                input.last_right_repeat = None;
                                input.held_right = Some(Instant::now());
                            } else {
                                input.last(None);
                            }
                        }
                    }
                    _ => {}
                },
                KeyCode::Right => match key.kind {
                    KeyEventKind::Press => {
                        if input.held_right.is_none() {
                            input.held_right = Some(Instant::now());
                            game.move_piece(Direction::Right);
                            input.last(Some(Direction::Right));
                        }
                    }
                    KeyEventKind::Release => {
                        input.last_right_repeat = None;
                        input.held_right = None;
                        if input.last_horizontal == Some(Direction::Right) {
                            if input.held_left.is_some() {
                                input.last(Some(Direction::Left));
                                input.last_left_repeat = None;
                                input.held_left = Some(Instant::now());
                            } else {
                                input.last(None);
                            }
                        }
                    }
                    _ => {}
                },
                KeyCode::Down => {
                    game.move_piece(Direction::Down);
                }
                KeyCode::Char('x') | KeyCode::Char('X') => {
                    if key.kind == KeyEventKind::Press {
                        game.rotate_piece(Rotation::Clockwise);
                    }
                }
                KeyCode::Char('z') | KeyCode::Char('Z') => {
                    if key.kind == KeyEventKind::Press {
                        game.rotate_piece(Rotation::CounterClockwise);
                    }
                }
                KeyCode::Modifier(ModifierKeyCode::LeftShift)
                | KeyCode::Modifier(ModifierKeyCode::RightShift) => {
                    if key.kind == KeyEventKind::Press {
                        game.rotate_piece(Rotation::HalfTurn);
                    }
                }
                KeyCode::Char('c') => {
                    if key.kind == KeyEventKind::Press {
                        game.hold();
                    }
                }
                KeyCode::Up if key.kind == KeyEventKind::Press => {
                    game.hard_drop();
                }
                KeyCode::Esc if key.kind == KeyEventKind::Press => game.end(),
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}
