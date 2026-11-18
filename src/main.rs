use rand::seq::SliceRandom;
use std::time::{Duration, Instant};
const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const O: Piece = Piece {
    shape: [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p: Cell::O,
};
const I: Piece = Piece {
    shape: [
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p: Cell::I,
};
const T: Piece = Piece {
    shape: [
        [false, true, false, false],
        [true, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p: Cell::T,
};
const S: Piece = Piece {
    shape: [
        [false, false, false, false],
        [false, true, true, false],
        [true, true, false, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p: Cell::S,
};
const Z: Piece = Piece {
    shape: [
        [false, false, false, false],
        [true, true, false, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p: Cell::Z,
};
const L: Piece = Piece {
    shape: [
        [false, false, false, false],
        [false, false, true, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p: Cell::L,
};
const J: Piece = Piece {
    shape: [
        [false, false, false, false],
        [true, false, false, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p: Cell::J,
};

fn main() {
    let mut pieces = [I, O, T, S, Z, L, J];
    let mut rng = rand::rng();
    pieces.shuffle(&mut rng);
    let mut game: Game = Game {
        board: [[Cell::Empty; WIDTH]; HEIGHT],
        current_piece: Piece {
            shape: pieces[0].shape,
            x: 3,
            y: 0,
            p: pieces[0].p,
        },
        pieces: pieces,
        next_piece_index: 1,
        game_over: false,
        score: 0,
    };
    let mut last_tick = Instant::now();
    while !game.game_over {
        if last_tick.elapsed() >= Duration::from_secs_f64(0.2) {
            render_game(&game);
            match tick(&mut game) {
                TickResult::Moved => println!("Moving"),
                TickResult::Landed => {
                    println!("Landed");
                    let lines = clear_lines(&mut game);
                    match lines {
                        1 => game.score += 100,
                        2 => game.score += 300,
                        3 => game.score += 500,
                        4 => game.score += 800,
                        _ => game.score += 0,
                    };
                    next_piece(&mut game);
                }
            };
            last_tick = Instant::now();
        }
    }
    println!("Game Over");
}
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
        println!();
    }
}
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

fn tick(game: &mut Game) -> TickResult {
    if can_move_down(game) {
        game.current_piece.y += 1;
        TickResult::Moved
    } else {
        lock_piece(game);
        TickResult::Landed
    }
}
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

fn can_move_down(game: &Game) -> bool {
    for y in 0..4 {
        let next_y = game.current_piece.y + y as i32 + 1;
        for x in 0..4 {
            let next_x = game.current_piece.x + x as i32;
            if game.current_piece.shape[y][x] {
                if (HEIGHT as i32 <= next_y)
                    || (HEIGHT as i32 >= next_y
                        && next_y >= 0
                        && game.board[next_y as usize][next_x as usize] != Cell::Empty)
                {
                    return false;
                }
            }
        }
    }
    return true;
}
fn can_move_left(game: &Game) -> bool {
    for y in 0..4 {
        let next_y = game.current_piece.y + y as i32;
        for x in 0..4 {
            let next_x = game.current_piece.x + x as i32 - 1;
            if game.current_piece.shape[y][x] {
                if (0 > next_x)
                    || (WIDTH as i32 > next_x
                        && next_x >= 0
                        && game.board[next_y as usize][next_x as usize] != Cell::Empty)
                {
                    return false;
                }
            }
        }
    }
    return true;
}
fn can_move_right(game: &Game) -> bool {
    for y in 0..4 {
        let next_y = game.current_piece.y + y as i32;
        for x in 0..4 {
            let next_x = game.current_piece.x + x as i32 + 1;
            if game.current_piece.shape[y][x] {
                if (WIDTH as i32 <= next_x)
                    || (WIDTH as i32 > next_x
                        && next_x >= 0
                        && game.board[next_y as usize][next_x as usize] != Cell::Empty)
                {
                    return false;
                }
            }
        }
    }
    return true;
}

fn move_left(game: &mut Game) {
    if can_move_left(game) {
        game.current_piece.x -= 1;
    }
}

fn move_right(game: &mut Game) {
    if can_move_right(game) {
        game.current_piece.x += 1;
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Cell {
    Empty,
    I,
    O,
    T,
    S,
    Z,
    L,
    J,
}
enum TickResult {
    Moved,
    Landed,
}

#[derive(Clone, Copy)]
struct Piece {
    shape: [[bool; 4]; 4],
    x: i32,
    y: i32,
    p: Cell,
}
struct Game {
    board: [[Cell; WIDTH]; HEIGHT],
    current_piece: Piece,
    pieces: [Piece; 7],
    next_piece_index: usize,
    game_over: bool,
    score: u32,
}
