const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const O_SHAPE: Piece = Piece {
    shape: [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
};
const I_SHAPE: Piece = Piece {
    shape: [
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
    ],
};
const T_SHAPE: Piece = Piece {
    shape: [
        [false, true, false, false],
        [true, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
};
const S_SHAPE: Piece = Piece {
    shape: [
        [false, false, false, false],
        [false, true, true, false],
        [true, true, false, false],
        [false, false, false, false],
    ],
};
const Z_SHAPE: Piece = Piece {
    shape: [
        [false, false, false, false],
        [true, true, false, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
};
const L_SHAPE: Piece = Piece {
    shape: [
        [false, false, false, false],
        [false, false, true, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
};
const J_SHAPE: Piece = Piece {
    shape: [
        [false, false, false, false],
        [true, false, false, false],
        [true, true, true, false],
        [false, false, false, false],
    ],
};

fn main() {
    let mut board = [[Cell::Empty; WIDTH]; HEIGHT];
    board[4][3] = Cell::I;
    render_board(&board);
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
}
