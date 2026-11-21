//Imports
use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use rand::seq::SliceRandom;
use std::time::{Duration, Instant};

mod pieces;
use pieces::{Cell, PIECES, Piece};

//Constants
const WIDTH: usize = 10;
const HEIGHT: usize = 20;

//Structs and Enums
#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Down,
}

enum TickResult {
    Moved,
    Landed,
}

struct Game {
    board: [[Cell; WIDTH]; HEIGHT],
    current_piece: Piece,
    pieces: [Piece; 7],
    next_piece_index: usize,
    game_over: bool,
    score: u32,
}

//Game Initialization
fn new_game() -> Game {
    let mut pieces = PIECES;
    pieces.shuffle(&mut rand::rng());

    Game {
        board: [[Cell::Empty; WIDTH]; HEIGHT],
        current_piece: Piece {
            shape: pieces[0].shape,
            x: 3,
            y: 0,
            p: pieces[0].p,
        },
        pieces,
        next_piece_index: 1,
        game_over: false,
        score: 0,
    }
}

//Game Logic

//Movement
fn move_piece(game: &mut Game, direction: Direction) -> bool {
    if can_move(game, direction) {
        let (dx, dy) = match direction {
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
        };
        game.current_piece.x += dx;
        game.current_piece.y += dy;
        return true;
    }
    false
}

fn tick(game: &mut Game) -> TickResult {
    if move_piece(game, Direction::Down) {
        TickResult::Moved
    } else {
        lock_piece(game);
        TickResult::Landed
    }
}

//Collision
fn can_move(game: &Game, direction: Direction) -> bool {
    let (dx, dy) = match direction {
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::Down => (0, 1),
    };
    for y in 0..4 {
        let next_y = game.current_piece.y + y as i32 + dy;
        for x in 0..4 {
            let next_x = game.current_piece.x + x as i32 + dx;
            if game.current_piece.shape[y][x] {
                if 0 > next_x
                    || WIDTH as i32 <= next_x
                    || 0 > next_y
                    || HEIGHT as i32 <= next_y
                    || game.board[next_y as usize][next_x as usize] != Cell::Empty
                {
                    return false;
                }
            }
        }
    }
    return true;
}

fn can_spawn(game: &Game, piece: &Piece) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if piece.shape[y][x]
                && game.board[piece.y as usize + y][piece.x as usize + x] != Cell::Empty
            {
                return false;
            }
        }
    }
    true
}

//Line Clearing
fn clear_lines(game: &mut Game) -> usize {
    let mut y = 0;
    let mut l: usize = 0;
    while y < HEIGHT {
        let mut c = 0;
        for x in 0..WIDTH {
            if game.board[y][x] != Cell::Empty {
                c += 1;
            }
        }
        if c == WIDTH {
            l += 1;
            for i in (1..=y).rev() {
                game.board[i] = game.board[i - 1];
            }
            game.board[0] = [Cell::Empty; WIDTH];
        } else {
            y += 1;
        }
    }
    l
}

//Piece Spawning
fn next_piece(game: &mut Game) {
    if game.next_piece_index >= game.pieces.len() {
        game.pieces.shuffle(&mut rand::rng());
        game.next_piece_index = 0;
    }
    let mut piece = game.pieces[game.next_piece_index];
    piece.x = 3;
    piece.y = 0;
    if can_spawn(game, &piece) {
        game.next_piece_index += 1;
        game.current_piece = Piece {
            shape: piece.shape,
            x: 3,
            y: 0,
            p: piece.p,
        };
    } else {
        game.game_over = true;
    }
}

fn lock_piece(game: &mut Game) {
    for y in 0..4 {
        for x in 0..4 {
            if game.current_piece.shape[y][x] {
                game.board[game.current_piece.y as usize + y][game.current_piece.x as usize + x] =
                    game.current_piece.p;
            }
        }
    }
}

//Updating Game
fn update_score(game: &mut Game, lines: usize) {
    match lines {
        1 => game.score += 100,
        2 => game.score += 300,
        3 => game.score += 500,
        4 => game.score += 800,
        _ => {}
    };
}
fn update_game(game: &mut Game) {
    match tick(game) {
        TickResult::Moved => {}
        TickResult::Landed => {
            let lines = clear_lines(game);
            update_score(game, lines);
            next_piece(game);
        }
    };
}

//Input
fn handle_input(game: &mut Game) -> std::io::Result<()> {
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

//Rendering
fn cell_to_char(cell: Cell) -> char {
    match cell {
        Cell::Empty => '.',
        Cell::I => 'I',
        Cell::O => 'O',
        Cell::T => 'T',
        Cell::S => 'S',
        Cell::Z => 'Z',
        Cell::L => 'L',
        Cell::J => 'J',
    }
}

fn render_game(game: &Game) {
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    print!("\r\n");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let piece_x = x as i32 - game.current_piece.x;
            let piece_y = y as i32 - game.current_piece.y;
            if piece_x >= 0 && piece_x <= 3 && piece_y >= 0 && piece_y <= 3 {
                if game.current_piece.shape[piece_y as usize][piece_x as usize] {
                    print!("{}", cell_to_char(game.current_piece.p));
                } else {
                    print!("{}", cell_to_char(game.board[y][x]));
                }
            } else {
                print!("{}", cell_to_char(game.board[y][x]));
            }
        }
        print!("\r\n");
    }
}

//Main
fn main() -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut game = new_game();
    let mut last_tick = Instant::now();
    while !game.game_over {
        handle_input(&mut game)?;
        if last_tick.elapsed() >= Duration::from_secs_f64(0.2) {
            update_game(&mut game);
            render_game(&game);
            last_tick = Instant::now();
        }
    }
    disable_raw_mode()?;
    println!("Game Over");
    Ok(())
}
