fn main() {
    const HEIGHT: usize = 20;
    const WIDTH: usize = 10;
    let mut board = [[0; WIDTH]; HEIGHT];
    board[4][3] = 1;
    for row in &board {
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
}
