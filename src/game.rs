//  game.rs
//  Imports
use crate::pieces::{Cell, Orientation, PIECES, Piece};
use rand::seq::SliceRandom;

//  Constants
const WIDTH: usize = 10;
const HEIGHT: usize = 20;

//  Types
#[derive(Clone, Copy)]
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
}
pub struct Game {
    board: [[Cell; WIDTH]; HEIGHT],
    current_piece: Piece,
    pieces: [Piece; 7],
    next_piece_index: usize,
    game_over: bool,
    score: u32,
    held_piece: Option<Piece>,
    hold_used: bool,
}
impl Game {
    // Create
    pub fn new() -> Game {
        let mut pieces = PIECES;
        pieces.shuffle(&mut rand::rng());

        Game {
            board: [[Cell::Empty; WIDTH]; HEIGHT],
            current_piece: Piece::new(
                pieces[0].shape(),
                3,
                0,
                pieces[0].cell(),
                pieces[0].orientation(),
            ),
            pieces,
            next_piece_index: 1,
            game_over: false,
            score: 0,
            held_piece: None,
            hold_used: false,
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
        if self.can_move(direction) {
            let (dx, dy) = match direction {
                Direction::Left => (-1, 0),
                Direction::Right => (1, 0),
                Direction::Down => (0, 1),
            };
            self.current_piece.move_by(dx, dy);
            return true;
        }
        false
    }
    fn can_move(&self, direction: Direction) -> bool {
        let (dx, dy) = match direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
        };
        for y in 0..4 {
            let next_y = self.current_piece.y() + y as i32 + dy;
            for x in 0..4 {
                let next_x = self.current_piece.x() + x as i32 + dx;
                if self.current_piece.shape()[y][x] {
                    if 0 > next_x
                        || WIDTH as i32 <= next_x
                        || 0 > next_y
                        || HEIGHT as i32 <= next_y
                        || self.board[next_y as usize][next_x as usize] != Cell::Empty
                    {
                        return false;
                    }
                }
            }
        }
        true
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
        }

        let new_orientation = rotated.orientation();

        let kicks: &[(i32, i32)] = match rotated.cell() {
            Cell::O => &[(0, 0)],

            Cell::I => {
                match (old_orientation, new_orientation) {
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
                    _ => &[(0, 0)],
                }
            }

            Cell::J | Cell::L | Cell::S | Cell::T | Cell::Z => {
                match (old_orientation, new_orientation) {
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
                    _ => &[(0, 0)],
                }
            }

            Cell::Empty => &[(0, 0)],
        };

        let original_x = rotated.x();
        let original_y = rotated.y();

        for &(dx, dy) in kicks {
            rotated.set_position(original_x, original_y);
            rotated.move_by(dx, dy);

            if self.can_spawn(&rotated) {
                self.current_piece = rotated;
                return;
            }
        }
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
    fn can_spawn(&self, piece: &Piece) -> bool {
        for y in 0..4 {
            let next_y = piece.y() + y as i32;
            for x in 0..4 {
                let next_x = piece.x() + x as i32;
                if piece.shape()[y][x] {
                    if next_x < 0
                        || next_x >= WIDTH as i32
                        || next_y < 0
                        || next_y >= HEIGHT as i32
                        || self.board[next_y as usize][next_x as usize] != Cell::Empty
                    {
                        return false;
                    }
                }
            }
        }
        true
    }
    fn next_piece(&mut self) {
        if self.next_piece_index >= self.pieces.len() {
            self.pieces.shuffle(&mut rand::rng());
            self.next_piece_index = 0;
        }
        let mut piece = self.pieces[self.next_piece_index];
        piece.set_position(3, 0);
        if self.can_spawn(&piece) {
            self.next_piece_index += 1;
            self.current_piece = Piece::new(piece.shape(), 3, 0, piece.cell(), Orientation::Spawn);
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
                let held_template = Self::get_template(piece.cell());
                // Save default pieces
                let temp = Piece::new(
                    curr_template.shape(),
                    3,
                    0,
                    curr_template.cell(),
                    Orientation::Spawn,
                );
                let check = Piece::new(
                    held_template.shape(),
                    3,
                    0,
                    held_template.cell(),
                    Orientation::Spawn,
                );
                if self.can_spawn(&check) {
                    self.current_piece = check;
                    self.held_piece = Some(temp);
                } else {
                    self.end();
                }
            }
            None => {
                let curr_template = Self::get_template(self.current_piece.cell());

                self.held_piece = Some(Piece::new(
                    curr_template.shape(),
                    3,
                    0,
                    curr_template.cell(),
                    Orientation::Spawn,
                ));
                self.next_piece();
            }
        }
        self.hold_used = true;
    }
    pub fn held_piece(&self) -> Option<&Piece> {
        self.held_piece.as_ref()
    }

    // Board management
    fn clear_lines(&mut self) -> usize {
        let mut y = 0;
        let mut l: usize = 0;
        while y < HEIGHT {
            let mut c = 0;
            for x in 0..WIDTH {
                if self.board[y][x] != Cell::Empty {
                    c += 1;
                }
            }
            if c == WIDTH {
                l += 1;
                for i in (1..=y).rev() {
                    self.board[i] = self.board[i - 1];
                }
                self.board[0] = [Cell::Empty; WIDTH];
            } else {
                y += 1;
            }
        }
        l
    }

    // Game update
    pub fn update(&mut self) {
        if self.game_over {
            return;
        }
        match self.tick() {
            TickResult::Moved => {}
            TickResult::Landed => self.finish_landing(),
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
            self.lock_piece();
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
// Find and return the original piece
