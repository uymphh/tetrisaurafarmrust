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
    let curr_piece = Piece {
        shape: O_SHAPE.shape,
        x: 3,
        y: 0,
        p_type: Cell::O,
    };
    render_game(&mut board, &curr_piece);
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
fn render_game(board: &mut [[Cell; WIDTH]; HEIGHT], piece: &Piece) {
    let mut h = 0;
    let mut k = 0;
    for y in piece.y..piece.y + 4 {
        for x in piece.x..piece.x + 4 {
            if piece.shape[h][k] {
                board[y as usize][x as usize] = piece.p_type;
            }
            k += 1;
        }
        k = 0;
        h += 1;
    }
    render_board(board);
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
