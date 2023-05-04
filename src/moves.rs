use crate::pieces::MoveType;
use crate::pieces::Move;
use crate::pieces::Piece;

pub fn new_move(piece: &Piece, new_index: usize, move_type : MoveType) -> Move {
    return Move {
        piece_id: piece.id, from: piece.index,
        to: new_index, move_type, swap_to: None
    };
}
