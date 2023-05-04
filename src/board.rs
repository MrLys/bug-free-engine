use string_builder::Builder;
use crate::pieces::Move;
use crate::pieces::Piece;
use crate::pieces::PieceTypes;

pub struct Board {
    pub board: Vec<Option<Piece>>,
    pub whites_turn: bool,
    pub moves: Vec<Move>
}
fn printable_board(board: &Board) -> Vec<String> {
    let mut builder = Builder::default();
    let mut row = 1;
    for j in 0..64 {
        let i = ((j-63 as i32).abs()) as usize;
        if j % 8 == 0 {
            if j != 0 {
                builder.append("\r");
            }
            builder.append(row.to_string());
            row += 1;
        }
        if let Some(piece) = &board.board[i] {
            builder.append(" ");
            builder.append(piece.to_string());
            builder.append(" ");
        } else {
            builder.append(" . ");
        }
        builder.append(" ");
    }
    builder.append("\r");
    builder.append(" ");
    let row_names = [" a ", " b ", " c ", " d ", " e ", " f ", " g ", " h "];
    builder.append(row_names.join(" "));

    return builder.string().unwrap().split("\r").map(str::to_string).collect::<Vec<String>>();
}
pub fn print_board(board: &Board) {
    let board_string = printable_board(&board);
    for row in board_string {
        println!("{:?}", row.to_string());
    }
}

impl Default for Board {
    fn default() -> Self {
        let board = vec![
            Some(Piece {piece_type: PieceTypes::ROOK, is_white: true, id: 1, index: 0, moved: false}),
            Some(Piece {piece_type: PieceTypes::KNIGHT, is_white: true, id: 2, index: 1, moved: false}),
            Some(Piece {piece_type: PieceTypes::BISHOP, is_white: true, id: 3, index: 2, moved: false}),
            Some(Piece {piece_type: PieceTypes::QUEEN, is_white: true, id: 4, index: 3, moved: false}),
            Some(Piece {piece_type: PieceTypes::KING, is_white: true, id: 5, index: 4, moved: false}),
            Some(Piece {piece_type: PieceTypes::BISHOP, is_white: true, id: 6, index: 5, moved: false}),
            Some(Piece {piece_type: PieceTypes::KNIGHT, is_white: true, id: 7, index: 6, moved: false}),
            Some(Piece {piece_type: PieceTypes::ROOK, is_white: true, id: 8, index: 7, moved: false}),

            Some(Piece {piece_type: PieceTypes::PAWN, is_white: true, id: 9, index: 8, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: true, id: 10, index: 9, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: true, id: 11, index: 10, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: true, id: 12, index: 11, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: true, id: 13, index: 12, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: true, id: 14, index: 13, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: true, id: 15, index: 14, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: true, id: 16, index: 15, moved: false}),

            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,

            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,

            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,

            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: false, id: 17, index: 47, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: false, id: 18, index: 48, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: false, id: 19, index: 49, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: false, id: 20, index: 50, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: false, id: 20, index: 51, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: false, id: 21, index: 52, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: false, id: 22, index: 53, moved: false}),
            Some(Piece {piece_type: PieceTypes::PAWN, is_white: false, id: 23, index: 54, moved: false}),

            Some(Piece {piece_type: PieceTypes::ROOK, is_white: false, id: 24, index: 55, moved: false}),
            Some(Piece {piece_type: PieceTypes::KNIGHT, is_white: false, id: 25, index: 56, moved: false}),
            Some(Piece {piece_type: PieceTypes::BISHOP, is_white: false, id: 26, index: 57, moved: false}),
            Some(Piece {piece_type: PieceTypes::QUEEN, is_white: false, id: 28, index: 58, moved: false}),
            Some(Piece {piece_type: PieceTypes::KING, is_white: false, id: 27, index: 59, moved: false}),
            Some(Piece {piece_type: PieceTypes::BISHOP, is_white: false, id: 29, index: 60, moved: false}),
            Some(Piece {piece_type: PieceTypes::KNIGHT, is_white: false, id: 30, index: 61, moved: false}),
            Some(Piece {piece_type: PieceTypes::ROOK, is_white: false, id: 31, index: 62, moved: false}),
            ];
        let whites_turn = true;
        return Board {
            board,
            whites_turn,
            moves: Vec::new()
        };
    }
}

