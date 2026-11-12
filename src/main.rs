fn main() {
    const HEIGHT: usize = 20;
    const WIDTH: usize = 10;
    let mut board = [[0; WIDTH]; HEIGHT];
    board[4][2] = 1;
    for line in &board {
        println!("{:?}", line)
    }
}
