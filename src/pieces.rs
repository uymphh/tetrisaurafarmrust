//pieces.rs
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
#[derive(PartialEq, Clone, Copy)]
pub enum Orientation {
    Spawn,
    Left,
    Reverse,
    Right,
}

#[derive(Clone, Copy)]
pub struct Piece {
    shape: [[bool; 4]; 4],
    x: i32,
    y: i32,
    cell: Cell,
    orientation: Orientation,
}
impl Piece {
    pub fn new(
        shape: [[bool; 4]; 4],
        x: i32,
        y: i32,
        cell: Cell,
        orientation: Orientation,
    ) -> Self {
        Self {
            shape,
            x,
            y,
            cell,
            orientation,
        }
    }
    pub fn x(&self) -> i32 {
        self.x
    }
    pub fn y(&self) -> i32 {
        self.y
    }
    pub fn shape(&self) -> [[bool; 4]; 4] {
        self.shape
    }
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }
    pub fn cell(&self) -> Cell {
        self.cell
    }
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
    pub fn rotate_clockwise(&mut self) {
        if self.cell != Cell::O {
            let mut new = [[false; 4]; 4];
            let m = match self.cell {
                Cell::I => 4,
                _ => 3,
            };
            for y in 0..m {
                for x in 0..m {
                    new[x][m - 1 - y] = self.shape[y][x];
                }
            }
            self.shape = new;
        }
        match self.orientation {
            Orientation::Spawn => self.orientation = Orientation::Right,
            Orientation::Right => self.orientation = Orientation::Reverse,
            Orientation::Reverse => self.orientation = Orientation::Left,
            Orientation::Left => self.orientation = Orientation::Spawn,
        }
    }
    pub fn rotate_counterclockwise(&mut self) {
        if self.cell != Cell::O {
            let mut new = [[false; 4]; 4];
            let m = match self.cell {
                Cell::I => 4,
                _ => 3,
            };
            for y in 0..m {
                for x in 0..m {
                    new[m - 1 - x][y] = self.shape[y][x];
                }
            }
            self.shape = new;
        }
        match self.orientation {
            Orientation::Spawn => self.orientation = Orientation::Left,
            Orientation::Left => self.orientation = Orientation::Reverse,
            Orientation::Reverse => self.orientation = Orientation::Right,
            Orientation::Right => self.orientation = Orientation::Spawn,
        }
    }
}

const O: Piece = Piece {
    shape: [
        [false, false, false, false],
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    cell: Cell::O,
    orientation: Orientation::Spawn,
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
    cell: Cell::I,
    orientation: Orientation::Spawn,
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
    cell: Cell::T,
    orientation: Orientation::Spawn,
};
const S: Piece = Piece {
    shape: [
        [false, true, true, false],
        [true, true, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    cell: Cell::S,
    orientation: Orientation::Spawn,
};
const Z: Piece = Piece {
    shape: [
        [true, true, false, false],
        [false, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    cell: Cell::Z,
    orientation: Orientation::Spawn,
};
const L: Piece = Piece {
    shape: [
        [false, false, true, false],
        [true, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    cell: Cell::L,
    orientation: Orientation::Spawn,
};
const J: Piece = Piece {
    shape: [
        [true, false, false, false],
        [true, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    x: 0,
    y: 0,
    cell: Cell::J,
    orientation: Orientation::Spawn,
};

pub const PIECES: [Piece; 7] = [I, O, T, S, Z, L, J];
