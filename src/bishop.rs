use crate::pieces::MoveType::{TAKE, MOVE};
use crate::pieces::Move;
use crate::pieces::Piece;
use crate::board::Board;
use crate::moves::new_move;
use crate::util::{index_to_row, index_to_column};

fn new_bishop_move(piece: &Piece, board: &Board, index: usize, moves: &mut Vec<Move>) -> bool{
    if let Some(p) = &board.board[index] {
        if (piece.is_white && !p.is_white) || (!piece.is_white && p.is_white) {
            moves.push(new_move(piece, index, TAKE));
        }
        return true;
    } else {
        moves.push(new_move(piece, index, MOVE));
        return false;
    }
}
pub fn valid_bishop_moves_in_vec(piece: &Piece, board: &Board, moves: &mut Vec<Move>) {
    let index = piece.index;
    valid_bishop_moves_up_right(piece, board, index,  moves);
    valid_bishop_moves_up_left(piece, board, index,  moves);
    valid_bishop_moves_down_left(piece, board, index,  moves);
    valid_bishop_moves_down_right(piece, board, index,  moves);
}
pub fn valid_bishop_moves(piece: &Piece, board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    valid_bishop_moves_in_vec(piece, board, &mut moves);
    return moves;
}

fn valid_bishop_moves_up_right(piece: &Piece, board: &Board, index: usize, moves: &mut Vec<Move>) {
    if index > 56 {
        return;
    }
    let mut new_index = index + 7;
    let mut row = index_to_row(new_index);
    while row <= 7 {
        if new_bishop_move(piece, board, new_index, moves) { return };
        new_index += 7;
        row = index_to_row(new_index);
    }
}

fn valid_bishop_moves_up_left(piece: &Piece, board: &Board, index: usize, moves: &mut Vec<Move>) {
    if index + 9 > 63 {
        return;
    }
    let mut new_index = index + 9;
    let mut row = index_to_row(new_index);
    let mut column = index_to_column(new_index);
    while row <= 7 && column <= 7 {
        if new_bishop_move(piece, board, new_index, moves) { return };
        new_index += 9;
        row = index_to_row(new_index);
        column = index_to_column(new_index);
    }
}

fn valid_bishop_moves_down_left(piece: &Piece, board: &Board, index: usize, moves: &mut Vec<Move>) {
    if index < 7 {
        return;
    }
    let mut new_index = index - 7;
    let mut column = index_to_column(new_index);
    while column <= 7 {
        println!("bishop.rs:68 column={:?}, new_index={:?}, index={:?}", column, new_index, index);
        if new_bishop_move(piece, board, new_index, moves) { return };
        new_index -= 7;
        column = index_to_column(new_index);
    }
}

fn valid_bishop_moves_down_right(piece: &Piece, board: &Board, index: usize, moves: &mut Vec<Move>) {
    if index < 9 {
        return;
    }
    let mut new_index = index - 9;
    let mut row = index_to_row(new_index);
    let mut column = index_to_column(new_index);
    while new_index >= 9 {
        if new_bishop_move(piece, board, new_index, moves) { return };
        new_index -= 9;
        row = index_to_row(new_index);
        column = index_to_column(new_index);
    }

}
