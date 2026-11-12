const WIDTH: usize = 10;
const HEIGHT: usize = 20;

fn main() {
    let mut board = [[0u8; WIDTH]; HEIGHT];
    board[4][3] = 1;
    render_board(&board);
}

fn render_board(board: &[[u8; WIDTH]; HEIGHT]) {
    println!("+==========+");
    for row in board {
        print!("|");
        for cell in row {
            match cell {
                0 => print!("."),
                1 => print!("#"),
                _ => print!("?"),
            }
        }
        println!("|");
    }
    println!("+==========+");
}
