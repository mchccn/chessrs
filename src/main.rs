#![allow(dead_code, non_upper_case_globals)]

mod board_representation;
mod game_state;
mod piece;

fn main() {
    println!("{}", game_state::GameState::Playing);
    println!("{}", piece::Piece::White | piece::Piece::Knight);
}
