use crate::bishop::valid_bishop_moves_in_vec;
use crate::pieces::Move;
use crate::pieces::Piece;
use crate::board::Board;
use crate::rook::valid_rook_moves_in_vec;

pub fn valid_queen_moves(piece: &Piece, board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    valid_bishop_moves_in_vec(piece, board, &mut moves);
    println!("{:?}", &moves.len());
    valid_rook_moves_in_vec(piece, board, &mut moves);
    return moves;
}

#[cfg(test)]
mod tests {
    use crate::{board::{Board, print_board}, pieces::{MovablePiece, MoveType}};


    #[test]
    fn base_queen_moves() {
        let board = &mut Board::default();
        let queen = board.board[3].as_ref().expect("There should be a queen here!");
        let moves = queen.valid_moves(board);
        assert_eq!(0, moves.len());
    }
    #[test]
    fn middle_queen_moves() {
        let board = &mut Board::default();
        board.board.swap(3, 27);
        board.board[27].as_mut().map(|p| {p.moved = true; p.index = 27;});
        let queen = board.board[27].as_ref().expect("There should be a queen here!");
        let moves = queen.valid_moves(&board);
        print_board(&board);
        assert_eq!(19, moves.len());

        //valid_bishop_moves_up_right(piece, board, index,  moves);
        assert_eq!(27+7, moves[0].to);
        assert_eq!(MoveType::MOVE, moves[0].move_type);

        assert_eq!(27+7+7, moves[1].to);
        assert_eq!(MoveType::MOVE, moves[1].move_type);

        assert_eq!(27+7+7+7, moves[2].to);
        assert_eq!(MoveType::TAKE, moves[2].move_type);

        //valid_bishop_moves_up_left(piece, board, index,  moves);
        assert_eq!(27+9, moves[3].to);
        assert_eq!(MoveType::MOVE, moves[3].move_type);

        assert_eq!(27+9+9, moves[4].to);
        assert_eq!(MoveType::MOVE, moves[4].move_type);

        assert_eq!(27+9+9+9, moves[5].to);
        assert_eq!(MoveType::TAKE, moves[5].move_type);

        //valid_bishop_moves_down_left(piece, board, index,  moves);
        assert_eq!(27-7, moves[6].to);
        assert_eq!(MoveType::MOVE, moves[6].move_type, "{:?}", &moves[6]);

        //valid_bishop_moves_down_right(piece, board, index,  moves);
        assert_eq!(27-9, moves[7].to);
        assert_eq!(MoveType::MOVE, moves[7].move_type);

        //valid_rook_moves_left(piece, board, index,  moves);
        assert_eq!(27+1, moves[8].to);
        assert_eq!(MoveType::MOVE, moves[8].move_type);

        assert_eq!(27+1+1, moves[9].to);
        assert_eq!(MoveType::MOVE, moves[9].move_type);

        assert_eq!(27+1+1+1, moves[10].to);
        assert_eq!(MoveType::MOVE, moves[10].move_type);

        assert_eq!(27+1+1+1+1, moves[11].to);
        assert_eq!(MoveType::MOVE, moves[11].move_type);

        //valid_rook_moves_right(piece, board, index,  moves);
        assert_eq!(27-1, moves[12].to);
        assert_eq!(MoveType::MOVE, moves[12].move_type);

        assert_eq!(27-1-1, moves[13].to);
        assert_eq!(MoveType::MOVE, moves[13].move_type);

        assert_eq!(27-1-1-1, moves[14].to);
        assert_eq!(MoveType::MOVE, moves[14].move_type);

        //valid_rook_moves_up(piece, board, index,  moves);
        assert_eq!(27+8, moves[15].to);
        assert_eq!(MoveType::MOVE, moves[15].move_type);

        assert_eq!(27+8+8, moves[16].to);
        assert_eq!(MoveType::MOVE, moves[16].move_type);

        assert_eq!(27+8+8+8, moves[17].to);
        assert_eq!(MoveType::TAKE, moves[17].move_type);

        //valid_rook_moves_down(piece, board, index,  moves);
        assert_eq!(27-8, moves[18].to);
        assert_eq!(MoveType::MOVE, moves[18].move_type);


    }
}
