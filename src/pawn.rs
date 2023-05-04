use crate::pieces::MoveType::{TAKE, MOVE, EnPassant, TakeSwap, SWAP};
use crate::pieces::Move;
use crate::pieces::Piece;
use crate::pieces::PieceTypes;
use crate::board::Board;
use crate::moves::new_move;

pub fn new_en_pessant(piece: &Piece, mv: &Move) -> Move {
    dbg!(piece);
    dbg!(mv);
    if piece.is_white {
        let new_index = if mv.to > piece.index {piece.index + 9} else {piece.index + 7};
        return Move {
            piece_id: piece.id,
            from: piece.index,
            to: new_index,
            move_type: EnPassant,
            swap_to: None,
        }
    } else {
        let new_index = if mv.to > piece.index {piece.index - 7} else {piece.index - 9};
        return Move {
            piece_id: piece.id,
            from: piece.index,
            to: new_index,
            move_type: EnPassant,
            swap_to: None,
        }
    }
}
fn is_first_pawn_move(from: usize, to: usize) -> bool {
    return (from as i32 - to as i32).abs() as usize > 8;
}
pub fn valid_pawn_moves(piece: &Piece, board: &Board) -> Vec<Move> {
    let mut moves = Vec::new();
    if piece.is_white {
        if piece.moved {
            let index = piece.index;
            // left takeswap
            if index > 47 && index < 55 {
                let new_index = index + 9;
                if let Some(p) = &board.board[new_index] {
                    if !p.is_white {
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::KNIGHT) });
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::BISHOP) });
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::QUEEN) });
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::ROOK) });
                    }
                }
            }
            // right takeswap
            if index > 48 && index <= 55 {
                let new_index = index + 7;
                if let Some(p) = &board.board[new_index] {
                    if !p.is_white {
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::KNIGHT) });
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::BISHOP) });
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::QUEEN) });
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::ROOK) });
                    }
                }
            }
            // straight swap
            if index > 47 && index <= 55 {
                let new_index = index + 8;
                if let Some(_p) = &board.board[new_index] {
                } else {
                    moves.push(Move { piece_id: piece.id, from: index,
                        to: new_index, move_type: SWAP, swap_to: Some(PieceTypes::KNIGHT) });
                    moves.push(Move { piece_id: piece.id, from: index,
                        to: new_index, move_type: SWAP, swap_to: Some(PieceTypes::BISHOP) });
                    moves.push(Move { piece_id: piece.id, from: index,
                        to: new_index, move_type: SWAP, swap_to: Some(PieceTypes::QUEEN) });
                    moves.push(Move { piece_id: piece.id, from: index,
                        to: new_index, move_type: SWAP, swap_to: Some(PieceTypes::ROOK) });
                }
            } else {
                // right
                if index % 8 > 0 {
                    let new_index = index + 7;
                    if let Some(_p) = &board.board[new_index] {
                        moves.push(new_move(piece, new_index, TAKE));
                    }
                }
                // left
                if index % 8 < 7 {
                    let new_index = index + 9;
                    if let Some(_p) = &board.board[new_index] {
                        moves.push(new_move(piece, new_index, TAKE));
                    }
                }
                // straight
                let new_index = index + 8;
                if let Some(_p) = &board.board[new_index] {
                } else {
                    let mv = new_move(piece, new_index, MOVE);
                    dbg!("white:",&mv);
                    moves.push(mv);
                }

            }
            if let Some(mv) = &board.moves.last() {
                match mv.move_type {
                    MOVE => if is_first_pawn_move(mv.from, mv.to) {moves.push(new_en_pessant(piece, &mv));} else {}
                    _ => {}
                }
            }
        } else {
            let index = piece.index;
            // right
            if index % 8 > 0 {
                let new_index = index + 7;
                if let Some(_p) = &board.board[new_index] {
                    moves.push(new_move(piece, new_index, TAKE));
                }
            }
            // left
            if index % 8 < 7 {
                let new_index = index + 9;
                if let Some(_p) = &board.board[new_index] {
                    moves.push(new_move(piece, new_index, TAKE));
                }
            }
            // straight
            let new_index = index + 8;
            if let Some(_p) = &board.board[new_index] {
            } else {
                moves.push(new_move(piece, new_index, MOVE));
            }
            let new_index = index + 16;
            if let Some(_p) = &board.board[new_index] {
            } else {
                moves.push(new_move(piece, new_index, MOVE));
            }
        }
    } else {
        if piece.moved {
            let index = piece.index;
            // left takeswap
            if index >= 8 && index < 15 {
                let new_index = index - 7;
                if let Some(p) = &board.board[new_index] {
                    if p.is_white {
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::KNIGHT) });
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::BISHOP) });
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::QUEEN) });
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::ROOK) });
                    }
                }
            }
            // right takeswap
            if index > 8 && index < 16 {
                let new_index = index - 9;
                if let Some(p) = &board.board[new_index] {
                    if p.is_white {
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::KNIGHT) });
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::BISHOP) });
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::QUEEN) });
                        moves.push(Move { piece_id: piece.id, from: index,
                            to: new_index, move_type: TakeSwap, swap_to: Some(PieceTypes::ROOK) });
                    }
                }
            }
            // straigt swap
            if index > 7 && index < 16 {
                let new_index = index - 8;
                if let Some(_p) = &board.board[new_index] {
                } else {
                    moves.push(Move { piece_id: piece.id, from: index,
                        to: new_index, move_type: SWAP, swap_to: Some(PieceTypes::KNIGHT) });
                    moves.push(Move { piece_id: piece.id, from: index,
                        to: new_index, move_type: SWAP, swap_to: Some(PieceTypes::BISHOP) });
                    moves.push(Move { piece_id: piece.id, from: index,
                        to: new_index, move_type: SWAP, swap_to: Some(PieceTypes::QUEEN) });
                    moves.push(Move { piece_id: piece.id, from: index,
                        to: new_index, move_type: SWAP, swap_to: Some(PieceTypes::ROOK) });
                }
            } else {
                // right
                if index % 8 > 0 {
                    let new_index = index - 9;
                    if let Some(_p) = &board.board[new_index] {
                        moves.push(new_move(piece, new_index, TAKE));
                    }
                }
                // left
                if index % 8 < 7 {
                    let new_index = index - 7;
                    if let Some(_p) = &board.board[new_index] {
                        moves.push(new_move(piece, new_index, TAKE));
                    }
                }
                // straight
                let new_index = index - 8;
                if let Some(_p) = &board.board[new_index] {
                } else {
                    let mv = new_move(piece, new_index, MOVE);
                    dbg!("black:",&mv);
                    moves.push(mv);
                }

            }
            if let Some(mv) = &board.moves.last() {
                match mv.move_type {
                    MOVE => if is_first_pawn_move(mv.from, mv.to) {moves.push(new_en_pessant(piece, &mv));} else {}
                    _ => {}
                }
            }
        } else {
            let index = piece.index;
            // right
            if index % 8 > 0 {
                let new_index = index - 9;
                if let Some(_p) = &board.board[new_index] {
                    moves.push(new_move(piece, new_index, TAKE));
                }
            }
            // left
            if index % 8 < 7 {
                let new_index = index - 7;
                if let Some(_p) = &board.board[new_index] {
                    moves.push(new_move(piece, new_index, TAKE));
                }
            }
            // straight
            let new_index = index - 8;
            if let Some(_p) = &board.board[new_index] {
            } else {
                moves.push(new_move(piece, new_index, MOVE));
            }
            let new_index = index;
            if let Some(_p) = &board.board[new_index] {
            } else {
                moves.push(new_move(piece, new_index, MOVE));
            }
        }
    }
    return moves;
}

#[cfg(test)]
mod tests {
    use crate::board::print_board;
    use crate::pieces::{Piece, PieceTypes, Move, MoveType};
    use crate::{board::Board, pieces::MovablePiece};
    use crate::pieces::MoveType::{TAKE, MOVE, EnPassant, TakeSwap, SWAP};

    #[test]
    fn base_moves() {
        let board = &mut Board::default();
        board.board[8].as_ref().map(|piece|{
            let moves = &piece.valid_moves(&board);
            assert_eq!(2, moves.len());
            dbg!(&moves);
            assert_eq!(8+8, moves[0].to);
            assert_eq!(MOVE, moves[0].move_type);
            assert_eq!(8+16, moves[1].to);
            assert_eq!(MOVE, moves[1].move_type);
        });
        board.board.swap(16,8);
        board.board[16].as_mut().map(|p| {p.moved = true; p.index = 16;});
        board.board[16].as_ref().map(|piece|{
            let moves = &piece.valid_moves(&board);
            assert_eq!(1, moves.len());
            assert_eq!(16+8, moves[0].to);
            assert_eq!(MOVE, moves[0].move_type);
        });

    }
    #[test]
    fn take_moves() {
        let board = &mut Board::default();
        board.board.swap(49,25);
        board.board[25].as_mut().map(|p| {p.moved = true; p.index = 25;});
        // this should not be regarded as a takeable pawn
        // as it's on the other side of the bord, but adjacent in terms of
        // indices
        board.board.swap(55,23);
        board.board[23].as_mut().map(|p| {p.moved = true; p.index = 23;});

        board.board.swap(16,8);
        board.board[16].as_mut().map(|p| {p.moved = true; p.index = 16;});

        board.board[16].as_ref().map(|piece|{
            let moves = &piece.valid_moves(&board);
            assert_eq!(2, moves.len());
            assert_eq!(16+9, moves[0].to);
            assert_eq!(TAKE, moves[0].move_type);
            assert_eq!(16+8, moves[1].to);
            assert_eq!(MOVE, moves[1].move_type);
        });

    }
    /**
     * This test checks that a pawn is able to make the expected moves and swap options when it reaches the opposite side of the board.
     *
     * The test does the following:
     * 1. Sets up a new chess board and removes the pieces at the end of the pawn's starting row and the row in front of it.
     * 2. Moves a pawn from its starting position to the row in front of it.
     * 3. Makes the pawn "moved" and sets its position to the opponent pawns starting square.
     * 4. Verifies that the pawn has 8 valid moves, including 4 that allow it to be swapped with a
     * knight, bishop, queen, or rook, and 4 that allow it to be swapped with any of those pieces.
     */
    #[test]
    fn swap_moves() {
        let board = &mut Board::default();
        board.board[56] = None;
        board.board[48] = None;

        board.board.swap(8,48);
        board.board[48].as_mut().map(|p| {p.moved = true; p.index = 48;});
        board.board[48].as_ref().map(|piece|{
            let moves = &piece.valid_moves(&board);
            dbg!(&moves);
            assert_eq!(8, moves.len());

            assert_eq!(57, moves[0].to);
            assert_eq!(TakeSwap, moves[0].move_type);
            moves[0].swap_to.as_ref().map(|m| assert_eq!(PieceTypes::KNIGHT, *m));

            assert_eq!(57, moves[1].to);
            assert_eq!(TakeSwap, moves[1].move_type);
            moves[1].swap_to.as_ref().map(|m| assert_eq!(PieceTypes::BISHOP, *m));

            assert_eq!(57, moves[2].to);
            assert_eq!(TakeSwap, moves[2].move_type);
            moves[2].swap_to.as_ref().map(|m| assert_eq!(PieceTypes::QUEEN, *m));

            assert_eq!(57, moves[3].to);
            assert_eq!(TakeSwap, moves[3].move_type);
            moves[3].swap_to.as_ref().map(|m| assert_eq!(PieceTypes::ROOK, *m));

            assert_eq!(56, moves[4].to);
            assert_eq!(SWAP, moves[4].move_type);
            moves[4].swap_to.as_ref().map(|m| assert_eq!(PieceTypes::KNIGHT, *m));

            assert_eq!(56, moves[5].to);
            assert_eq!(SWAP, moves[5].move_type);
            moves[5].swap_to.as_ref().map(|m| assert_eq!(PieceTypes::BISHOP, *m));

            assert_eq!(56, moves[6].to);
            assert_eq!(SWAP, moves[6].move_type);
            moves[6].swap_to.as_ref().map(|m| assert_eq!(PieceTypes::QUEEN, *m));

            assert_eq!(56, moves[7].to);
            assert_eq!(SWAP, moves[7].move_type);
            moves[7].swap_to.as_ref().map(|m| assert_eq!(PieceTypes::ROOK, *m));
        });
    }
    #[test]
    fn swap_take_moves() {
    }
    #[test]
    fn en_passant_moves() {
        let board = &mut Board::default();
        board.board.swap(24,8);
        board.board.swap(25,55);
        board.board[25].as_mut().map(|p| {p.moved = true; p.index = 25;});
        board.board[8] = None;
        board.board[55] = None;
        board.moves.push(Move {
            piece_id: 9,
            from: 8,
            to: 24,
            move_type: MoveType::MOVE,
            swap_to: None
        });
        if let Some(piece) = &board.board[25] {
            print_board(board);
            let moves = piece.valid_moves(&board);
            assert_eq!(2, moves.len());
            assert_eq!(17, moves[0].to);
            assert_eq!(MOVE, moves[0].move_type);

            assert_eq!(16, moves[1].to);
            assert_eq!(EnPassant, moves[1].move_type);
        } else {
            assert!(false);
        }
    }
}
