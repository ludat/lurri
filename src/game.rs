#[macro_export]
macro_rules! piece(
    ($color:pat, $piece:pat) => (
        $crate::game::Piece { tipo: $piece, color: $color }
    );
);

pub fn error() {
    let piece = Piece { color: White , tipo: King };
    match piece {
        piece!(color, pt @ King) | piece!(color, pt @ Knight) => {
            println!("This is the responsable line I think")
        },
    }
}

use self::Color::{White, Black};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

use self::PieceType::{King, Queen, Rook, Bishop, Knight, Pawn};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Piece {
    pub tipo: PieceType,
    pub color: Color,
}
