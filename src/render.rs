//  render.rs
use crate::game::Game;
use crate::pieces::Cell;
const WIDTH: usize = 10;
const HEIGHT: usize = 20;
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
//  Rendering
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

pub fn render_game(game: &Game) {
    execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    print!("\r\n");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let piece_x = x as i32 - game.current_piece().x();
            let piece_y = y as i32 - game.current_piece().y();
            if piece_x >= 0 && piece_x <= 3 && piece_y >= 0 && piece_y <= 3 {
                if game.current_piece().shape()[piece_y as usize][piece_x as usize] {
                    print!("{}", cell_to_char(game.current_piece().cell()));
                } else {
                    print!("{}", cell_to_char(game.board()[y][x]));
                }
            } else {
                print!("{}", cell_to_char(game.board()[y][x]));
            }
        }
        print!("\r\n");
    }
    print!("\r\n");
    print!("Hold:");
    print!("\r\n");
    if let Some(piece) = game.held_piece() {
        for y in 0..4 {
            for x in 0..4 {
                if piece.shape()[y][x] {
                    print!("{}", cell_to_char(piece.cell()))
                } else {
                    print!(".")
                }
            }
            print!("\r\n");
        }
    } else {
        print!("Empty");
        print!("\r\n");
    }
}
