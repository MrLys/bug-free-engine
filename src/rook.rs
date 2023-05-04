use crate::moves::new_move;
use crate::pieces::MoveType;
use crate::pieces::Move;
use crate::pieces::Piece;
use crate::board::Board;
use crate::util::index_to_row;

fn valid_rook_moves_left(piece: &Piece, board: &Board, index: usize, moves: &mut Vec<Move>) {
    let mut new_index = index + 1;
    println!("rook.rs:19 new_index={:?} %8={:?}", new_index,  new_index % 8);
    while new_index % 8 > 0 && new_index % 8 <= 8 {
        if let Some(p) = &board.board[new_index as usize] {
            if (piece.is_white && !p.is_white) || (!piece.is_white && p.is_white){
                moves.push(new_move(piece, new_index as usize , MoveType::TAKE));
            }
            return;
        } else {
            moves.push(new_move(piece, new_index as usize , MoveType::MOVE));
        }
        new_index += 1;
    }
}
fn valid_rook_moves_right(piece: &Piece, board: &Board, index: usize, moves: &mut Vec<Move>) {
    if index < 1 {
        return;
    }
    let mut new_index = index -1;
    while new_index % 8 > 0 && new_index % 8 <= 8 {
        if let Some(p) = &board.board[new_index as usize] {
            if (piece.is_white && !p.is_white) || (!piece.is_white && p.is_white){
                moves.push(new_move(piece, new_index as usize , MoveType::TAKE));
            }
            return;
        } else {
            moves.push(new_move(piece, new_index as usize , MoveType::MOVE));
        }
        new_index -= 1 ;
    }
}
fn valid_rook_moves_up(piece: &Piece, board: &Board, index: usize, moves: &mut Vec<Move>) {
    if index > 55 {
        return;
    }
    let mut new_index = index + 8;
    let row = index_to_row(new_index);
    while new_index > 0 && row < 6 {
        if let Some(p) = &board.board[new_index as usize] {
            if (piece.is_white && !p.is_white) || (!piece.is_white && p.is_white){
                moves.push(new_move(piece, new_index as usize , MoveType::TAKE));
            }
            return;
        } else {
            moves.push(new_move(piece, new_index as usize , MoveType::MOVE));
        }
        new_index = new_index + 8;
    }
}
fn valid_rook_moves_down(piece: &Piece, board: &Board, index: usize, moves: &mut Vec<Move>) {
    if index < 8 {
        return;
    }
    let mut new_index = index - 8;
    let row = index_to_row(new_index);
    while new_index > 0 && row > 0 {
        if let Some(p) = &board.board[new_index as usize] {
            if (piece.is_white && !p.is_white) || (!piece.is_white && p.is_white){
                moves.push(new_move(piece, new_index as usize , MoveType::TAKE));
            }
            return;
        } else {
            moves.push(new_move(piece, new_index as usize , MoveType::MOVE));
        }
        new_index = new_index - 8;
    }
}
pub fn valid_rook_moves_in_vec(piece: &Piece, board: &Board, moves: &mut Vec<Move>) {
    let index = piece.index;
    valid_rook_moves_left(piece, board, index, moves);
    valid_rook_moves_right(piece, board, index, moves);
    valid_rook_moves_up(piece, board, index, moves);
    valid_rook_moves_down(piece, board, index, moves);
}
pub fn valid_rook_moves(piece: &Piece, board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    let index = piece.index;
    valid_rook_moves_left(piece, board, index, &mut moves);
    valid_rook_moves_right(piece, board, index, &mut moves);
    valid_rook_moves_up(piece, board, index, &mut moves);
    valid_rook_moves_down(piece, board, index, &mut moves);
    return moves;
}
