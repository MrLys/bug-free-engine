mod pieces;
mod board;
mod pawn;
mod rook;
mod knight;
mod util;
mod moves;
mod bishop;
mod queen;

use pieces::{Move, PieceTypes};

use crate::pieces::MovablePiece;
use std::time::SystemTime;
fn main() {
    let board = board::Board::default();
    let mut all_moves : Vec<Move> = Vec::new();
    for p in &board.board {
        if let Some(piece) = p {
            match &piece.piece_type {
                PieceTypes::ROOK => {
                    let mut duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                    let start = duration_since_epoch.as_nanos(); // u128
                    all_moves.append(&mut piece.valid_moves(&board));
                    duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                    let end = duration_since_epoch.as_nanos(); // u128
                    println!("ROOK::valid_moves took {:?} ns", end - start);
                },
                PieceTypes::KNIGHT => {
                    let mut duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                    let start = duration_since_epoch.as_nanos(); // u128
                    all_moves.append(&mut piece.valid_moves(&board));
                    duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                    let end = duration_since_epoch.as_nanos(); // u128
                    println!("KNIGHT::valid_moves took {:?} ns", end - start);
                },
                PieceTypes::PAWN => {
                    let mut duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                    let start = duration_since_epoch.as_nanos(); // u128
                    all_moves.append(&mut piece.valid_moves(&board));
                    duration_since_epoch = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                    let end = duration_since_epoch.as_nanos(); // u128
                    println!("PAWN::valid_moves took {:?} ns", end - start);
                },
                _=> {},

            }
        }
    }
}
