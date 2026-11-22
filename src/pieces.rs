//pieces.rs
#[derive(Clone, Copy)]
pub struct Piece {
    pub shape: [[bool; 4]; 4],
    pub x: i32,
    pub y: i32,
    pub p: Cell,
}
#[derive(PartialEq, Clone, Copy)]
pub enum Cell {
    Empty,
    I,
    O,
    T,
    S,
    Z,
    L,
    J,
}
pub const PIECES: [Piece; 7] = [I, O, T, S, Z, L, J];

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
