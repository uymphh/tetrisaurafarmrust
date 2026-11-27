//  game.rs
//  Imports
use crate::pieces::{Cell, Orientation, PIECES, Piece};
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use std::time::Instant;

//  Constants
const WIDTH: usize = 10;
const HEIGHT: usize = 20;

//  Types
#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Down,
}
enum TickResult {
    Moved,
    Landed,
}
pub enum Rotation {
    Clockwise,
    CounterClockwise,
    HalfTurn,
}
pub struct Game {
    board: [[Cell; WIDTH]; HEIGHT],
    current_piece: Piece,
    piece_queue: VecDeque<Piece>,
    next_bag: VecDeque<Piece>,
    game_over: bool,
    score: u32,
    held_piece: Option<Piece>,
    hold_used: bool,
    ground_at: Option<Instant>,
    lock_reset: u8,
}
impl Game {
    // Create
    pub fn new() -> Game {
        let mut piece_queue = PIECES;
        let mut next_bag = PIECES;

        piece_queue.shuffle(&mut rand::rng());
        next_bag.shuffle(&mut rand::rng());
        let mut piece_queue: VecDeque<Piece> = piece_queue.into();
        let next_bag: VecDeque<Piece> = next_bag.into();
        let first_piece = piece_queue.pop_front().expect("Bag is empty");

        Game {
            board: [[Cell::Empty; WIDTH]; HEIGHT],
            current_piece: Piece::new(
                first_piece.shape(),
                3,
                0,
                first_piece.cell(),
                first_piece.orientation(),
            ),
            piece_queue,
            next_bag,
            game_over: false,
            score: 0,
            held_piece: None,
            hold_used: false,
            ground_at: None,
            lock_reset: 0,
        }
    }

    // Information
    fn get_template(cell: Cell) -> Piece {
        PIECES
            .iter()
            .copied()
            .find(|temp| temp.cell() == cell)
            .expect("Piece template not found")
    }
    pub fn next_pieces(&self) -> impl Iterator<Item = &Piece> {
        self.piece_queue.iter().take(5)
    }
    pub fn end(&mut self) {
        self.game_over = true;
    }
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
    pub fn current_piece(&self) -> &Piece {
        &self.current_piece
    }
    pub fn board(&self) -> &[[Cell; WIDTH]; HEIGHT] {
        &self.board
    }
    pub fn score(&self) -> u32 {
        self.score
    }

    // Movement
    pub fn move_piece(&mut self, direction: Direction) -> bool {
        if self.game_over {
            return false;
        }
        if self.can_move(&self.current_piece, direction) {
            let (dx, dy) = match direction {
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
                Direction::Down => (0, 1),
            };
            let grounded = !self.can_move(&self.current_piece, Direction::Down);
            self.current_piece.move_by(dx, dy);
            if matches!(direction, Direction::Left | Direction::Right) {
                if self.lock_reset < 15 && grounded {
                    self.ground_at = None;
                    self.lock_reset += 1;
                }
            }
            return true;
        }
        false
    }
    fn can_move(&self, piece: &Piece, direction: Direction) -> bool {
        let (dx, dy) = match direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
        };
        let mut moved = *piece;
        moved.move_by(dx, dy);
        self.can_place(&moved)
    }
    pub fn rotate_piece(&mut self, rotation: Rotation) {
        if self.game_over {
            return;
        }

        let old_orientation = self.current_piece.orientation();
        let mut rotated = self.current_piece;

        match rotation {
            Rotation::Clockwise => rotated.rotate_clockwise(),
            Rotation::CounterClockwise => rotated.rotate_counterclockwise(),
            Rotation::HalfTurn => {
                rotated.rotate_counterclockwise();
                rotated.rotate_counterclockwise();
            }
        };

        let new_orientation = rotated.orientation();
        let kicks = get_kicks(rotated.cell(), old_orientation, new_orientation);
        self.try_rotation(&mut rotated, kicks);
    }
    fn try_rotation(&mut self, rotated: &mut Piece, kicks: &[(i32, i32)]) -> bool {
        let original_x = rotated.x();
        let original_y = rotated.y();

        for &(dx, dy) in kicks {
            rotated.set_position(original_x, original_y);
            rotated.move_by(dx, dy);

            if self.can_place(&rotated) {
                let grounded = !self.can_move(&self.current_piece, Direction::Down);
                self.current_piece = *rotated;
                if self.lock_reset < 15 && grounded {
                    self.ground_at = None;
                    self.lock_reset += 1;
                }
                return true;
            }
        }
        false
    }
    pub fn hard_drop(&mut self) {
        if self.game_over {
            return;
        }
        while self.move_piece(Direction::Down) {}

        self.lock_piece();
        self.finish_landing();
    }

    // Piece management
    fn can_place(&self, piece: &Piece) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if !piece.shape()[y][x] {
                    continue;
                }
                let board_x = piece.x() + x as i32;
                let board_y = piece.y() + y as i32;
                if board_x < 0
                    || board_x >= WIDTH as i32
                    || board_y < 0
                    || board_y >= HEIGHT as i32
                    || self.board[board_y as usize][board_x as usize] != Cell::Empty
                {
                    return false;
                }
            }
        }
        true
    }
    fn refill_bag(&mut self) {
        if self.piece_queue.len() < 6 {
            if self.next_bag.is_empty() {
                let mut next_bag = PIECES;

                next_bag.shuffle(&mut rand::rng());
                self.next_bag = next_bag.into();
            }
            self.piece_queue.append(&mut self.next_bag);
        }
    }

    fn next_piece(&mut self) {
        self.lock_reset = 0;
        self.ground_at = None;
        self.refill_bag();
        let mut piece = self.piece_queue.pop_front().expect("No pieces available");
        piece.set_position(3, 0);
        if self.can_place(&piece) {
            self.current_piece = piece;
        } else {
            self.end();
        }
    }
    fn lock_piece(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                if self.current_piece.shape()[y][x] {
                    self.board[(self.current_piece.y() + y as i32) as usize]
                        [(self.current_piece.x() + x as i32) as usize] = self.current_piece.cell();
                }
            }
        }
    }
    pub fn hold(&mut self) {
        if self.is_game_over() || self.hold_used {
            return;
        }
        match self.held_piece {
            Some(piece) => {
                // Get default pieces
                let curr_template = Self::get_template(self.current_piece.cell());
                let mut next = piece;
                next.set_position(3, 0);
                // Save default pieces
                if self.can_place(&next) {
                    self.current_piece = next;
                    self.held_piece = Some(curr_template);
                } else {
                    self.end();
                }
            }
            None => {
                let curr_template = Self::get_template(self.current_piece.cell());

                self.held_piece = Some(curr_template);
                self.next_piece();
            }
        }
        self.hold_used = true;
        self.ground_at = None;
        self.lock_reset = 0;
    }
    pub fn held_piece(&self) -> Option<&Piece> {
        self.held_piece.as_ref()
    }
    pub fn ghost_piece(&self) -> Piece {
        let mut ghost = self.current_piece;
        while self.can_move(&ghost, Direction::Down) {
            ghost.move_by(0, 1);
        }
        ghost
    }

    // Board management
    fn clear_lines(&mut self) -> usize {
        let mut y = 0;
        let mut lines_cleared: usize = 0;
        while y < HEIGHT {
            let full_line = self.board[y].iter().all(|x| *x != Cell::Empty);
            if full_line {
                lines_cleared += 1;
                for row in (1..=y).rev() {
                    self.board[row] = self.board[row - 1];
                }
                self.board[0] = [Cell::Empty; WIDTH];
            } else {
                y += 1;
            }
        }
        lines_cleared
    }

    // Game update
    pub fn update(&mut self) {
        if self.game_over {
            return;
        }
        match self.tick() {
            TickResult::Moved => {
                self.ground_at = None;
            }
            TickResult::Landed => {
                if let Some(ground_at) = self.ground_at {
                    if ground_at.elapsed().as_millis() >= 500 {
                        self.lock_piece();
                        self.finish_landing();
                    }
                } else {
                    self.ground_at = Some(Instant::now());
                }
            }
        };
    }
    fn finish_landing(&mut self) {
        let lines = self.clear_lines();
        self.update_score(lines);
        self.hold_used = false;
        self.next_piece();
    }
    fn tick(&mut self) -> TickResult {
        if self.move_piece(Direction::Down) {
            TickResult::Moved
        } else {
            TickResult::Landed
        }
    }

    // Score
    fn update_score(&mut self, lines: usize) {
        match lines {
            1 => self.score += 100,
            2 => self.score += 300,
            3 => self.score += 500,
            4 => self.score += 800,
            _ => {}
        };
    }
}
fn get_kicks(cell: Cell, old: Orientation, new: Orientation) -> &'static [(i32, i32)] {
    match cell {
        Cell::O => &[(0, 0)],

        Cell::I => {
            match (old, new) {
                // Clockwise
                (Orientation::Spawn, Orientation::Right) => {
                    &[(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]
                }

                (Orientation::Right, Orientation::Reverse) => {
                    &[(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)]
                }

                (Orientation::Reverse, Orientation::Left) => {
                    &[(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]
                }

                (Orientation::Left, Orientation::Spawn) => {
                    &[(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]
                }

                // Counterclockwise
                (Orientation::Spawn, Orientation::Left) => {
                    &[(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)]
                }

                (Orientation::Left, Orientation::Reverse) => {
                    &[(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)]
                }

                (Orientation::Reverse, Orientation::Right) => {
                    &[(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)]
                }

                (Orientation::Right, Orientation::Spawn) => {
                    &[(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)]
                }
                // 180-degree rotation
                (Orientation::Spawn, Orientation::Reverse)
                | (Orientation::Reverse, Orientation::Spawn)
                | (Orientation::Left, Orientation::Right)
                | (Orientation::Right, Orientation::Left) => {
                    &[(0, 0), (0, -1), (-1, 0), (1, 0), (0, -2), (0, 1)]
                }
                _ => &[(0, 0)],
            }
        }

        Cell::J | Cell::L | Cell::S | Cell::T | Cell::Z => {
            match (old, new) {
                // Clockwise
                (Orientation::Spawn, Orientation::Right) => {
                    &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
                }

                (Orientation::Right, Orientation::Reverse) => {
                    &[(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]
                }

                (Orientation::Reverse, Orientation::Left) => {
                    &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]
                }

                (Orientation::Left, Orientation::Spawn) => {
                    &[(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]
                }

                // Counterclockwise
                (Orientation::Spawn, Orientation::Left) => {
                    &[(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)]
                }

                (Orientation::Left, Orientation::Reverse) => {
                    &[(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)]
                }

                (Orientation::Reverse, Orientation::Right) => {
                    &[(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)]
                }

                (Orientation::Right, Orientation::Spawn) => {
                    &[(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)]
                }
                // 180-degree rotation
                (Orientation::Spawn, Orientation::Reverse)
                | (Orientation::Reverse, Orientation::Spawn)
                | (Orientation::Left, Orientation::Right)
                | (Orientation::Right, Orientation::Left) => {
                    &[(0, 0), (0, -1), (-1, 0), (1, 0), (0, -2), (0, 1)]
                }
                _ => &[(0, 0)],
            }
        }

        Cell::Empty => &[(0, 0)],
    }
}
