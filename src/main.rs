const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const O_SHAPE: Piece = Piece {
    shape: [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p_type: Cell::O,
};
const I_SHAPE: Piece = Piece {
    shape: [
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p_type: Cell::I,
};
const T_SHAPE: Piece = Piece {
    shape: [
        [false, true, false, false],
        [true, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p_type: Cell::T,
};
const S_SHAPE: Piece = Piece {
    shape: [
        [false, false, false, false],
        [false, true, true, false],
        [true, true, false, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p_type: Cell::S,
};
const Z_SHAPE: Piece = Piece {
    shape: [
        [false, false, false, false],
        [true, true, false, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p_type: Cell::Z,
};
const L_SHAPE: Piece = Piece {
    shape: [
        [false, false, false, false],
        [false, false, true, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p_type: Cell::L,
};
const J_SHAPE: Piece = Piece {
    shape: [
        [false, false, false, false],
        [true, false, false, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    p_type: Cell::J,
};

fn main() {
    let mut board = [[Cell::Empty; WIDTH]; HEIGHT];
    let mut curr_piece = Piece {
        shape: O_SHAPE.shape,
        x: 3,
        y: 0,
        p_type: Cell::O,
    };
    render_game(&board, &curr_piece);
}

fn render_board(board: &[[crate::Cell; WIDTH]; HEIGHT]) {
    println!("+==========+");
    for row in board {
        print!("|");
        for cell in row {
            match cell {
                Cell::Empty => print!("."),
                Cell::I => print!("I"),
                Cell::O => print!("O"),
                Cell::T => print!("T"),
                Cell::S => print!("S"),
                Cell::Z => print!("Z"),
                Cell::L => print!("L"),
                Cell::J => print!("J"),
            }
        }
        println!("|");
    }
    println!("+==========+");
}

fn render_piece(piece: &Piece) {
    for row in piece.shape {
        for cell in row {
            match cell {
                true => print!("#"),
                false => print!("."),
            };
        }
        println!("");
    }
}
fn render_game(board: &[[Cell; WIDTH]; HEIGHT], piece: &Piece) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let piece_x = x as i32 - piece.x;
            let piece_y = y as i32 - piece.y;
            if piece_x >= 0 && piece_x <= 3 && piece_y >= 0 && piece_y <= 3 {
                if piece.shape[piece_y as usize][piece_x as usize] {
                    match piece.p_type {
                        Cell::Empty => print!("."),
                        Cell::I => print!("I"),
                        Cell::O => print!("O"),
                        Cell::T => print!("T"),
                        Cell::S => print!("S"),
                        Cell::Z => print!("Z"),
                        Cell::L => print!("L"),
                        Cell::J => print!("J"),
                    }
                } else {
                    match board[y][x] {
                        Cell::Empty => print!("."),
                        Cell::I => print!("I"),
                        Cell::O => print!("O"),
                        Cell::T => print!("T"),
                        Cell::S => print!("S"),
                        Cell::Z => print!("Z"),
                        Cell::L => print!("L"),
                        Cell::J => print!("J"),
                    }
                }
            } else {
                match board[y][x] {
                    Cell::Empty => print!("."),
                    Cell::I => print!("I"),
                    Cell::O => print!("O"),
                    Cell::T => print!("T"),
                    Cell::S => print!("S"),
                    Cell::Z => print!("Z"),
                    Cell::L => print!("L"),
                    Cell::J => print!("J"),
                }
            }
        }
        println!();
    }
}
#[derive(Clone, Copy)]
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

struct Piece {
    shape: [[bool; 4]; 4],
    x: i32,
    y: i32,
    p_type: Cell,
}
