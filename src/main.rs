const WIDTH: usize = 10;
const HEIGHT: usize = 20;

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
