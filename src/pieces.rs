use crate::bishop::{valid_bishop_moves, valid_bishop_moves_in_vec};
use crate::knight::valid_knight_moves;
use crate::pawn::valid_pawn_moves;
use crate::board::Board;
use crate::queen::valid_queen_moves;
use crate::rook::valid_rook_moves;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum PieceTypes {
    ROOK,
    BISHOP,
    KNIGHT,
    QUEEN,
    KING,
    PAWN
}

#[derive(Debug)]
pub struct Piece {
    pub id: usize,
    pub piece_type: PieceTypes,
    pub is_white: bool,
    pub moved: bool,
    pub index: usize
}

impl ToString for Piece {

    fn to_string(&self) -> String {
        return match self.piece_type {
            PieceTypes::ROOK => if self.is_white {"\u{2656}"} else {"\u{265C}"},
            PieceTypes::BISHOP => if self.is_white {"\u{2657}"} else {"\u{265D}"},
            PieceTypes::KNIGHT => if self.is_white {"\u{2658}"} else {"\u{265E}"},
            PieceTypes::QUEEN => if self.is_white {"\u{2655}"} else {"\u{265B}"},
            PieceTypes::KING => if self.is_white {"\u{2654}"} else {"\u{265A}"},
            PieceTypes::PAWN => if self.is_white {"\u{2659}"} else {"\u{265F}"},
        }.to_string();

    }
}
impl MovablePiece for Piece {
    fn valid_moves(&self, board: &Board) -> Vec<Move> {
        return match self.piece_type {
           PieceTypes::ROOK => valid_rook_moves(self, board),
           PieceTypes::BISHOP => valid_bishop_moves(self, board),
           PieceTypes::KNIGHT => valid_knight_moves(self, board),
           PieceTypes::QUEEN => valid_queen_moves(self, board),
           PieceTypes::KING => todo!(),
           PieceTypes::PAWN => valid_pawn_moves(self, board),
        };
    }
}
pub trait MovablePiece {
    fn valid_moves(&self, board: &Board) -> Vec<Move>;
}


#[derive(Debug)]
#[derive(PartialEq)]
pub enum MoveType {
    SWAP,
    TAKE,
    MOVE,
    TakeSwap,
    EnPassant,
    Castle
}

#[derive(Debug)]
pub struct Move {
    pub piece_id: usize,
    pub from: usize,
    pub to: usize,
    pub move_type: MoveType,
    pub swap_to: Option<PieceTypes>
}
