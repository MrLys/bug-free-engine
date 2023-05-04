use crate::moves::new_move;
use crate::pieces::Move;
use crate::board::Board;
use crate::pieces::Piece;
use crate::pieces::MoveType::{TAKE, MOVE};
use crate::util::{index_to_row, index_to_column};

fn create_knight_move(piece: &Piece, board: &Board, moves: &mut Vec<Move>, new_index: usize) {
    if let Some(p) = &board.board[new_index as usize] {
        if (piece.is_white && !p.is_white) || (!piece.is_white && p.is_white) {
        moves.push(new_move(piece, new_index, TAKE ));
        }
    } else {
        moves.push(new_move(piece, new_index, MOVE));
    }
}
pub fn valid_knight_moves(piece: &Piece, board: &Board) -> Vec<Move> {
    let mut moves : Vec<Move> = Vec::new();
    //[17, 10, -6, -15, -17, -10, 6, 15];
    let index = piece.index;
    let row = index_to_row(piece.index);
    let column = index_to_column(piece.index);

    // _
    //  |
    //  |
    if column < 7 && row < 6 {
        let new_index = index + 17;
        create_knight_move(piece, board, &mut moves, new_index);
    }
    //
    // i__
    //
    if column < 6 && row < 7 {
        let new_index = index + 10;
        create_knight_move(piece, board, &mut moves, new_index);
    }
    //
    // __
    // ^
    if column < 6 && row > 0 {
        let new_index = (index as i32 - 6) as usize;
        create_knight_move(piece, board, &mut moves, new_index);
    }
    //  |
    // _|
    //
    if column < 7 && row > 3 {
        let new_index = (index as i32 - 15) as usize;
        create_knight_move(piece, board, &mut moves, new_index);
    }
    //  |
    //  |_
    //
    if column > 0 && row > 3 {
        let new_index = (index as i32 - 17) as usize;
        create_knight_move(piece, board, &mut moves, new_index);
    }
    //
    //  __
    //   ^
    if column > 3 && row > 0 {
        let new_index = (index as i32 - 10) as usize;
        create_knight_move(piece, board, &mut moves, new_index);
    }
    //
    //  __i
    //
    if column > 3 && row < 7 {
        let new_index = index + 6;
        create_knight_move(piece, board, &mut moves, new_index);
    }
    //  _
    // |
    // |

    if column > 0 && row < 6 {
        let new_index = index + 15;
        create_knight_move(piece, board, &mut moves, new_index);
    }
    return moves;
}

#[cfg(test)]
mod test {
    use crate::{board::Board, pieces::MovablePiece};

    #[test]
    fn knight_move_test_base() {
        let board = Board::default();

        let knight1 = board.board[1].as_ref().expect("There should be a queen here!");
        let moves = knight1.valid_moves(&board);
        assert_eq!(2, moves.len());
    }

    #[test]
    fn knight_move_test_advanced() {
        let board = Board::default();
        board.board[1].as_ref().map(|piece|{
            let moves = &piece.valid_moves(&board);
            assert_eq!(2, moves.len());
        });
    }
}
