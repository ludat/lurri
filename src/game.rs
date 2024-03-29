#![allow(dead_code)]
use std::fmt;

use std::cmp::{Eq, Ord, Ordering, PartialOrd};

use std::ops::{Add, Neg, Not};

extern crate bit_vec;

const BASE_MOVEMENT_CAPACITY: usize = 100;

const KING_DELTAS: &'static [PositionDelta; 8] = &[
    PositionDelta { x: 1, y: 1 },
    PositionDelta { x: 1, y: 0 },
    PositionDelta { x: 1, y: -1 },
    PositionDelta { x: -1, y: 1 },
    PositionDelta { x: -1, y: 0 },
    PositionDelta { x: -1, y: -1 },
    PositionDelta { x: 0, y: 1 },
    PositionDelta { x: 0, y: -1 },
];
const KNIGHT_DELTAS: &'static [PositionDelta; 8] = &[
    PositionDelta { x: 1, y: 2 },
    PositionDelta { x: -1, y: 2 },
    PositionDelta { x: 1, y: -2 },
    PositionDelta { x: -1, y: -2 },
    PositionDelta { x: 2, y: 1 },
    PositionDelta { x: 2, y: -1 },
    PositionDelta { x: -2, y: 1 },
    PositionDelta { x: -2, y: -1 },
];

const QUEEN_DIRS: &'static [Direction; 8] =
    &[Up, Down, Right, Left, UpRight, UpLeft, DownRight, DownLeft];

const ROOK_DIRS: &'static [Direction; 4] = &[Up, Down, Right, Left];

const BISHOP_DIRS: &'static [Direction; 4] = &[UpRight, UpLeft, DownRight, DownLeft];

#[macro_export]
macro_rules! piece(
    ($color:pat, $piece:pat) => (
        $crate::game::Piece { tipo: $piece, color: $color }
    );
);

pub type Board = [[Option<Square>; 12]; 12];

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub turn: Color,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [
                [
                    None, None, None, None, None, None, None, None, None, None, None, None,
                ],
                [
                    None, None, None, None, None, None, None, None, None, None, None, None,
                ],
                [
                    // 1
                    None,
                    None,
                    Some(Square::white_rook()),
                    Some(Square::white_knight()),
                    Some(Square::white_bishop()),
                    Some(Square::white_queen()),
                    Some(Square::white_king()),
                    Some(Square::white_bishop()),
                    Some(Square::white_knight()),
                    Some(Square::white_rook()),
                    None,
                    None,
                ],
                [
                    // 2
                    None,
                    None,
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    None,
                    None,
                ],
                [
                    // 3
                    None,
                    None,
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    None,
                    None,
                ],
                [
                    // 4
                    None,
                    None,
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    None,
                    None,
                ],
                [
                    // 5
                    None,
                    None,
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    None,
                    None,
                ],
                [
                    // 6
                    None,
                    None,
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    None,
                    None,
                ],
                [
                    // 7
                    None,
                    None,
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    None,
                    None,
                ],
                [
                    // 8
                    None,
                    None,
                    Some(Square::black_rook()),
                    Some(Square::black_knight()),
                    Some(Square::black_bishop()),
                    Some(Square::black_queen()),
                    Some(Square::black_king()),
                    Some(Square::black_bishop()),
                    Some(Square::black_knight()),
                    Some(Square::black_rook()),
                    None,
                    None,
                ],
                [
                    None, None, None, None, None, None, None, None, None, None, None, None,
                ],
                [
                    None, None, None, None, None, None, None, None, None, None, None, None,
                ],
            ],
            turn: White,
        }
    }
    pub fn show(&self) {
        println!("{}", self)
    }
    pub fn get_raw_square(&self, pos: Position) -> Option<Square> {
        self.board[pos.y as usize][pos.x as usize]
    }
    pub fn get_square(&self, pos: Position) -> Square {
        self.get_raw_square(pos).unwrap()
    }
    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        self.get_square(pos).content
    }
    pub fn set_raw_square(
        &mut self,
        pos: Position,
        square: Option<Square>,
    ) -> Result<(), &'static str> {
        Ok(self.board[pos.y as usize][pos.x as usize] = square)
    }
    pub fn set_square(&mut self, pos: Position, piece: Option<Piece>) -> Result<(), &'static str> {
        self.set_raw_square(pos, Some(Square::new(piece)))
    }
    pub fn is_square(&self, pos: Position) -> bool {
        self.get_raw_square(pos).is_some()
    }
    pub fn get_to_by(&self, mov: &Move, dir: Direction) -> bool {
        let mut p: Position = mov.from;
        loop {
            p = p.go(dir);
            if p == mov.to {
                return true;
            };
            match self.get_raw_square(p) {
                Some(Square { content: None }) => continue,
                _ => return false,
            }
        }
    }
    pub fn make_move(&mut self, m: &Move) -> Result<(), &'static str> {
        match m.tipo {
            MoveType::Normal => {
                self.is_valid_normal_move(m)?;
                self.raw_make_move(m)?;
                Ok(())
            }
            MoveType::Promotion(pt) => {
                let color = self.turn;
                self.is_valid_normal_move(m)?;
                self.raw_make_move(m)?; // BEWARE: This changes the color
                self.set_square(m.to, Some(Piece::new(color, pt)))?;
                Ok(())
            }
            MoveType::LongCastling => match self.turn {
                White => {
                    let king_mov = Move::new(
                        Position::safe_from_chars('e', '1'),
                        Position::safe_from_chars('c', '1'),
                        MoveType::Normal,
                    );
                    let rook_mov = Move::new(
                        Position::safe_from_chars('a', '1'),
                        Position::safe_from_chars('d', '1'),
                        MoveType::Normal,
                    );
                    if self.get_square(rook_mov.from) == Square::white_rook()
                        && self.get_square(king_mov.from) == Square::white_king()
                        && self.get_to_by(
                            &Move::new(king_mov.from, rook_mov.from, MoveType::Normal),
                            Left,
                        )
                    {
                        self.raw_move(&rook_mov)?;
                        self.raw_make_move(&king_mov)?;
                        Ok(())
                    } else {
                        Err("Bad long casting")
                    }
                }
                Black => {
                    let king_mov = Move::new(
                        Position::safe_from_chars('e', '8'),
                        Position::safe_from_chars('c', '8'),
                        MoveType::Normal,
                    );
                    let rook_mov = Move::new(
                        Position::safe_from_chars('a', '8'),
                        Position::safe_from_chars('d', '8'),
                        MoveType::Normal,
                    );
                    if self.get_square(rook_mov.from) == Square::white_rook()
                        && self.get_square(king_mov.from) == Square::white_king()
                        && self.get_to_by(
                            &Move::new(king_mov.from, rook_mov.from, MoveType::Normal),
                            Left,
                        )
                    {
                        self.raw_move(&rook_mov)?;
                        self.raw_make_move(&king_mov)?;
                        Ok(())
                    } else {
                        Err("Bad long casting")
                    }
                }
            },
            MoveType::ShortCastling => match self.turn {
                White => {
                    let king_mov = Move::new(
                        Position::safe_from_chars('e', '1'),
                        Position::safe_from_chars('g', '1'),
                        MoveType::Normal,
                    );
                    let rook_mov = Move::new(
                        Position::safe_from_chars('h', '1'),
                        Position::safe_from_chars('f', '1'),
                        MoveType::Normal,
                    );
                    if self.get_square(rook_mov.from) == Square::white_rook()
                        && self.get_square(king_mov.from) == Square::white_king()
                        && self.get_to_by(
                            &Move::new(king_mov.from, rook_mov.from, MoveType::Normal),
                            Right,
                        )
                    {
                        self.raw_move(&rook_mov)?;
                        self.raw_make_move(&king_mov)?;
                        Ok(())
                    } else {
                        Err("Bad short casting")
                    }
                }
                Black => {
                    let king_mov = Move::new(
                        Position::safe_from_chars('e', '8'),
                        Position::safe_from_chars('g', '8'),
                        MoveType::Normal,
                    );
                    let rook_mov = Move::new(
                        Position::safe_from_chars('h', '8'),
                        Position::safe_from_chars('f', '8'),
                        MoveType::Normal,
                    );
                    if self.get_square(rook_mov.from) == Square::black_rook()
                        && self.get_square(king_mov.from) == Square::black_king()
                        && self.get_to_by(
                            &Move::new(king_mov.from, rook_mov.from, MoveType::Normal),
                            Right,
                        )
                    {
                        self.raw_move(&rook_mov)?;
                        self.raw_make_move(&king_mov)?;
                        Ok(())
                    } else {
                        Err("Bad short casting")
                    }
                }
            },
        }
    }
    pub fn is_valid_normal_move(&self, m: &Move) -> Result<(), &'static str> {
        match (self.get_raw_square(m.from), self.get_raw_square(m.to)) {
            (None, _) => Err("Not even a valid square"),
            (_, None) => Err("Not even a valid square"),
            (Some(Square { content: None }), _) => Err("Empty from square"),
            (Some(from_square), Some(to_square)) => {
                if from_square.get_piece().color != self.turn {
                    return Err("Wrong color");
                };
                match from_square.get_piece() {
                    piece!(color, King) => {
                        if (m.from.up() == m.to
                            || m.from.down() == m.to
                            || m.from.left() == m.to
                            || m.from.right() == m.to
                            || m.from.up().right() == m.to
                            || m.from.up().left() == m.to
                            || m.from.down().right() == m.to
                            || m.from.down().left() == m.to)
                            && !to_square.has_color(color)
                        {
                            Ok(())
                        } else {
                            Err("Bad King movement")
                        }
                    }
                    piece!(color, Queen) => {
                        if (self.get_to_by(m, UpRight)
                            || self.get_to_by(m, UpLeft)
                            || self.get_to_by(m, DownRight)
                            || self.get_to_by(m, DownLeft)
                            || self.get_to_by(m, Up)
                            || self.get_to_by(m, Down)
                            || self.get_to_by(m, Right)
                            || self.get_to_by(m, Left))
                            && !to_square.has_color(color)
                        {
                            Ok(())
                        } else {
                            Err("Bad Queen movement")
                        }
                    }
                    piece!(color, Rook) => {
                        if (self.get_to_by(m, Up)
                            || self.get_to_by(m, Down)
                            || self.get_to_by(m, Right)
                            || self.get_to_by(m, Left))
                            && !to_square.has_color(color)
                        {
                            Ok(())
                        } else {
                            Err("Bad Rook movement")
                        }
                    }
                    piece!(color, Bishop) => {
                        if (self.get_to_by(m, UpRight)
                            || self.get_to_by(m, UpLeft)
                            || self.get_to_by(m, DownRight)
                            || self.get_to_by(m, DownLeft))
                            && !to_square.has_color(color)
                        {
                            Ok(())
                        } else {
                            Err("Bad Bishop movement")
                        }
                    }
                    piece!(color, Knight) => {
                        if (m.from.up().up().right() == m.to
                            || m.from.up().up().left() == m.to
                            || m.from.down().down().right() == m.to
                            || m.from.down().down().left() == m.to
                            || m.from.right().right().up() == m.to
                            || m.from.right().right().down() == m.to
                            || m.from.left().left().up() == m.to
                            || m.from.left().left().down() == m.to)
                            && !to_square.has_color(color)
                        {
                            Ok(())
                        } else {
                            Err("Bad Knight movement")
                        }
                    }
                    piece!(color, Pawn) => {
                        let (promotion_y, long_move_y, foward_dir) = match color {
                            White => (Position::ch2y('7'), Position::ch2y('2'), Up),
                            Black => (Position::ch2y('2'), Position::ch2y('7'), Down),
                        };
                        if m.from.y == promotion_y && !m.is_promotion() {
                            Err("You must promote that pawn")
                        } else if (m.from.y == long_move_y
                            && m.from.go(foward_dir).go(foward_dir) == m.to
                            && to_square.has_none()
                            && self.get_to_by(m, foward_dir))
                            || (m.from.go(foward_dir) == m.to && to_square.has_none())
                            || ((m.from.go(foward_dir).left() == m.to
                                || m.from.go(foward_dir).right() == m.to)
                                && to_square.has_color(!color))
                        {
                            Ok(())
                        } else {
                            Err("Bad pawn movement")
                        }
                    }
                }
            }
        }
    }
    pub fn raw_make_move(&mut self, m: &Move) -> Result<(), &'static str> {
        let mut aux = self.clone();
        aux.raw_move(m)?;
        aux.turn = !aux.turn;
        if aux.can_eat_king() {
            return Err("The king can be eaten after that move");
        }
        self.raw_move(m)?;
        self.turn = !self.turn;
        Ok(())
    }
    pub fn raw_move(&mut self, m: &Move) -> Result<(), &'static str> {
        let from_piece = self.get_piece(m.from);
        self.set_square(m.to, from_piece)?;
        self.set_square(m.from, None)?;
        Ok(())
    }
    pub fn get_all_valid_moves(&self) -> Vec<ValuedMove> {
        let mut moves = Vec::with_capacity(BASE_MOVEMENT_CAPACITY);
        for from_pos in Position::all() {
            if self.get_square(from_pos).has_color(self.turn) {
                self.get_valid_moves(from_pos, &mut moves);
            };
        }
        moves
    }
    pub fn get_valid_moves<'a>(
        &self,
        from_pos: Position,
        moves: &'a mut Vec<ValuedMove>,
    ) -> &'a mut Vec<ValuedMove> {
        let piece = match self.get_raw_square(from_pos) {
            Some(Square {
                content: Some(piece),
            }) => piece,
            _ => return moves,
        };
        match piece {
            Piece {
                color,
                tipo: pt @ King,
            }
            | Piece {
                color,
                tipo: pt @ Knight,
            } => {
                for delta_pos in pt.get_posible_deltas().iter() {
                    let to_pos = from_pos + *delta_pos;
                    if let Some(to_square) = self.get_raw_square(to_pos) {
                        if !to_square.has_color(color) {
                            moves.push(ValuedMove::new(from_pos, to_pos, MoveType::Normal));
                        }
                    }
                }
            }
            Piece {
                color,
                tipo: pt @ Rook,
            }
            | Piece {
                color,
                tipo: pt @ Bishop,
            }
            | Piece {
                color,
                tipo: pt @ Queen,
            } => {
                for dir in pt.get_posible_dirs().iter() {
                    for to_pos in from_pos.iter_to(*dir) {
                        match self.get_raw_square(to_pos) {
                            Some(Square {
                                content: Some(piece!(to_color, _)),
                            }) => {
                                if color != to_color {
                                    moves.push(ValuedMove::new(from_pos, to_pos, MoveType::Normal));
                                }
                                break;
                            }
                            Some(Square { content: None }) => {
                                moves.push(ValuedMove::new(from_pos, to_pos, MoveType::Normal));
                            }
                            _ => break,
                        }
                    }
                }
            }
            piece!(color, Pawn) => {
                let (promotion_y, long_move_y, foward_dir) = match color {
                    White => (Position::ch2y('7'), Position::ch2y('2'), Up),
                    Black => (Position::ch2y('2'), Position::ch2y('7'), Down),
                };
                for to_pos in [
                    from_pos.go(foward_dir).left(),
                    from_pos.go(foward_dir).right(),
                ]
                .iter()
                {
                    if let Some(to_square) = self.get_raw_square(*to_pos) {
                        if to_square.has_color(!color) {
                            if from_pos.y == promotion_y {
                                for promotion_piece in [Queen, Rook, Bishop, Knight].iter() {
                                    moves.push(ValuedMove::new(
                                        from_pos,
                                        *to_pos,
                                        MoveType::Promotion(*promotion_piece),
                                    ));
                                }
                            } else {
                                moves.push(ValuedMove::new(from_pos, *to_pos, MoveType::Normal))
                            }
                        }
                    }
                }
                let to_pos = from_pos.go(foward_dir);
                if let Some(to_square) = self.get_raw_square(to_pos) {
                    if to_square.has_none() {
                        if from_pos.y == promotion_y {
                            for promotion_piece in [Queen, Rook, Bishop, Knight].iter() {
                                moves.push(ValuedMove::new(
                                    from_pos,
                                    to_pos,
                                    MoveType::Promotion(*promotion_piece),
                                ));
                            }
                        } else {
                            moves.push(ValuedMove::new(from_pos, to_pos, MoveType::Normal))
                        }
                        let to_pos = to_pos.go(foward_dir);
                        if let Some(to_square) = self.get_raw_square(to_pos) {
                            if from_pos.y == long_move_y && to_square.has_none() {
                                moves.push(ValuedMove::new(from_pos, to_pos, MoveType::Normal));
                            }
                        }
                    }
                }
            }
        };
        moves
    }
    pub fn can_be_eaten_by(&self, from_pos: Position, color: Color) -> bool {
        for &pt in [King, Queen, Rook, Bishop, Knight, Pawn].iter() {
            match pt {
                King | Knight => {
                    for delta_pos in pt.get_posible_deltas().iter() {
                        let to_pos = from_pos + *delta_pos;
                        if let Some(to_square) = self.get_raw_square(to_pos) {
                            if to_square.contains(Piece::new(color, pt)) {
                                return true;
                            }
                        }
                    }
                }
                Rook | Bishop | Queen => {
                    for dir in pt.get_posible_dirs().iter() {
                        for to_pos in from_pos.iter_to(*dir) {
                            if let Some(to_square) = self.get_raw_square(to_pos) {
                                if to_square.contains(Piece::new(color, pt)) {
                                    return true;
                                } else if to_square.has_none() {
                                    continue;
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
                Pawn => {
                    let backward_dir = match color {
                        White => Down,
                        Black => Up,
                    };
                    for &to_pos in [
                        from_pos.go(backward_dir).left(),
                        from_pos.go(backward_dir).right(),
                    ]
                    .iter()
                    {
                        if let Some(to_square) = self.get_raw_square(to_pos) {
                            if to_square.contains(Piece::new(color, pt)) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
    pub fn can_eat_king(&self) -> bool {
        let color = self.turn;
        for pos in Position::all() {
            let square = self.get_square(pos);
            if square.contains(Piece::new(!color, King)) {
                return self.can_be_eaten_by(pos, color);
            }
        }
        unreachable!("There is no king on the board")
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} Plays\n", self.turn)?;
        write!(f, "    a b c d e f g h  \n")?;
        write!(f, "  +-----------------+\n")?;
        for r in (2..10).rev() {
            write!(f, "{} |", r - 1)?;
            for c in 2..10 {
                match self.board[r][c] {
                    Some(ref p) => write!(f, " {}", p)?,
                    None => write!(f, "")?,
                };
            }
            write!(f, " | {}\n", r - 1)?;
        }
        write!(f, "  +-----------------+\n")?;
        write!(f, "    a b c d e f g h  \n")?;
        Ok(())
    }
}

// #[bench]
// fn bench_is_valid(b: &mut test::Bencher) {
//     let mut game: Game = Game::new();
//     b.iter(|| {
//         let mut game: Game = Game::new();
//         game.make_move(&Move::safe_from_string("e2e4"))
//     });
// }

#[test]
fn test_helper_functions() {
    let mut game = Game::new();
    let pos = Position::safe_from_chars('e', '2');
    assert!(!game.is_square(Position::new(0, 0)));
    assert!(game.is_square(pos));
    assert_eq!(game.get_square(pos), Square::white_pawn());
    assert!(game.make_move(&Move::safe_from_string("e2e4")).is_ok());
    assert!(game.make_move(&Move::safe_from_string("e2d4")).is_err());
}
#[test]
fn test_game_1() {
    let mut game = Game::new();
    assert!(game.make_move(&Move::safe_from_string("e2e4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("e7e5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g1f3")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("b8c6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f1c4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f8c5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c2c3")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c5b6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d2d4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d8e7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("O-O")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g8f6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d4d5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c6b8")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c4d3")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d7d6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("b1d2")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("a7a6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d2c4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("b6a7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("a2a4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("O-O")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("b2b4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f6e8")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d1c2")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g7g6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c1h6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("e8g7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c4e3")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f7f6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("a1e1")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f8f7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g1h1")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("b8d7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g2g4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d7f8")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f1g1")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("a7e3")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f2e3")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c8d7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g1g3")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c7c6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d3c4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c6d5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c4d5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d7e6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("e1g1")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("a8c8")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f3h4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("e6d5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("e4d5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c8c7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("h4f5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g6f5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g4f5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("e7e8")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c2g2")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("e8d7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g3g7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f7g7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("h6g7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d7g7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g2c2")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f8g6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f5g6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("h7h6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c2f5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g7f8")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c3c4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g8g7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g1c1")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("b7b6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("e3e4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f8e7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f5f2")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c7b7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("h2h4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("a6a5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("h4h5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("a5b4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c1b1")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("b4b3")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("b1b3")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("e7d7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f2f5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d7e7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f5e6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("e7c7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("e6f7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c7f7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g6f7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("b7a7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("b3b6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("a7a4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("b6d6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("a4c4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d6f6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("g7f8")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d5d6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("c4e4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d6d7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("e4d4")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f6h6")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("f8f7")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("h6h8")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d4d5")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d7d8q")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("d5d8")).is_ok());
    game.show();
    assert!(game.make_move(&Move::safe_from_string("h8d8")).is_ok());
    game.show();
}

use self::Color::{Black, White};
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn get_sign(&self) -> i32 {
        match *self {
            White => 1,
            Black => -1,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                White => "White",
                Black => "Black",
            }
        )
    }
}

impl Not for Color {
    type Output = Color;
    fn not(self) -> Color {
        match self {
            White => Black,
            Black => White,
        }
    }
}

#[test]
fn color_not() {
    assert_eq!(Black, !White);
    assert_eq!(White, !Black);
}

use self::PieceType::{Bishop, King, Knight, Pawn, Queen, Rook};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceType {
    pub fn from_char(c: char) -> Result<PieceType, &'static str> {
        match c {
            'r' => Ok(Rook),
            'n' => Ok(Knight),
            'b' => Ok(Bishop),
            'q' => Ok(Queen),
            'k' => Ok(King),
            'p' => Ok(Pawn),
            _ => Err("Not a valid piece type"),
        }
    }
    pub fn safe_from_char(c: char) -> PieceType {
        PieceType::from_char(c).unwrap()
    }
    pub fn get_value(&self) -> i32 {
        match *self {
            King => 0,
            Queen => 9,
            Rook => 5,
            Bishop => 3,
            Knight => 3,
            Pawn => 1,
        }
    }

    pub fn get_posible_dirs(&self) -> &'static [Direction] {
        match *self {
            Queen => QUEEN_DIRS,
            Rook => ROOK_DIRS,
            Bishop => BISHOP_DIRS,
            _ => unreachable!(
                "Asked posible directions of a piece that doesn't have posible directions"
            ),
        }
    }
    pub fn get_posible_deltas(&self) -> &'static [PositionDelta] {
        match *self {
            King => KING_DELTAS,
            Knight => KNIGHT_DELTAS,
            _ => unreachable!(
                "Asked posible positions of a piece that doesn't have defined positions"
            ),
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Rook => 'r',
                Knight => 'n',
                Bishop => 'b',
                Queen => 'q',
                King => 'k',
                Pawn => 'p',
            }
        )
    }
}

#[test]
fn piecetype_from_char() {
    assert_eq!(PieceType::safe_from_char('r'), Rook);
    assert_eq!(PieceType::safe_from_char('n'), Knight);
    assert_eq!(PieceType::safe_from_char('b'), Bishop);
    assert_eq!(PieceType::safe_from_char('q'), Queen);
    assert_eq!(PieceType::safe_from_char('k'), King);
    assert_eq!(PieceType::safe_from_char('p'), Pawn);
    assert!(PieceType::from_char('j').is_err());
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Piece {
    pub tipo: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn new(c: Color, t: PieceType) -> Piece {
        Piece { tipo: t, color: c }
    }
    pub fn get_value(&self) -> i32 {
        self.color.get_sign() * self.tipo.get_value()
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                piece!(Black, Rook) => 'R',
                piece!(Black, Knight) => 'N',
                piece!(Black, Bishop) => 'B',
                piece!(Black, Queen) => 'Q',
                piece!(Black, King) => 'K',
                piece!(Black, Pawn) => 'P',

                piece!(White, Rook) => 'r',
                piece!(White, Knight) => 'n',
                piece!(White, Bishop) => 'b',
                piece!(White, Queen) => 'q',
                piece!(White, King) => 'k',
                piece!(White, Pawn) => 'p',
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square {
    pub content: Option<Piece>,
}

impl Square {
    pub fn new(p: Option<Piece>) -> Square {
        Square { content: p }
    }

    pub fn black_rook() -> Square {
        Square {
            content: Some(Piece {
                tipo: Rook,
                color: Black,
            }),
        }
    }
    pub fn black_knight() -> Square {
        Square {
            content: Some(Piece {
                tipo: Knight,
                color: Black,
            }),
        }
    }
    pub fn black_bishop() -> Square {
        Square {
            content: Some(Piece {
                tipo: Bishop,
                color: Black,
            }),
        }
    }
    pub fn black_queen() -> Square {
        Square {
            content: Some(Piece {
                tipo: Queen,
                color: Black,
            }),
        }
    }
    pub fn black_king() -> Square {
        Square {
            content: Some(Piece {
                tipo: King,
                color: Black,
            }),
        }
    }
    pub fn black_pawn() -> Square {
        Square {
            content: Some(Piece {
                tipo: Pawn,
                color: Black,
            }),
        }
    }

    pub fn white_rook() -> Square {
        Square {
            content: Some(Piece {
                tipo: Rook,
                color: White,
            }),
        }
    }
    pub fn white_knight() -> Square {
        Square {
            content: Some(Piece {
                tipo: Knight,
                color: White,
            }),
        }
    }
    pub fn white_bishop() -> Square {
        Square {
            content: Some(Piece {
                tipo: Bishop,
                color: White,
            }),
        }
    }
    pub fn white_queen() -> Square {
        Square {
            content: Some(Piece {
                tipo: Queen,
                color: White,
            }),
        }
    }
    pub fn white_king() -> Square {
        Square {
            content: Some(Piece {
                tipo: King,
                color: White,
            }),
        }
    }
    pub fn white_pawn() -> Square {
        Square {
            content: Some(Piece {
                tipo: Pawn,
                color: White,
            }),
        }
    }
    pub fn empty() -> Square {
        Square { content: None }
    }

    pub fn get_piece(&self) -> Piece {
        self.content.unwrap()
    }

    pub fn has_color(&self, color: Color) -> bool {
        match (color, *self) {
            (
                White,
                Square {
                    content: Some(piece!(White, _)),
                },
            ) => true,
            (
                Black,
                Square {
                    content: Some(piece!(Black, _)),
                },
            ) => true,
            _ => false,
        }
    }
    pub fn contains(&self, piece: Piece) -> bool {
        match *self {
            Square { content: Some(p) } if p == piece => true,
            _ => false,
        }
    }

    pub fn has_white(&self) -> bool {
        match *self {
            Square {
                content: Some(piece!(White, _)),
            } => true,
            _ => false,
        }
    }
    pub fn has_black(&self) -> bool {
        match *self {
            Square {
                content: Some(piece!(Black, _)),
            } => true,
            _ => false,
        }
    }
    pub fn has_none(&self) -> bool {
        match *self {
            Square { content: None } => true,
            _ => false,
        }
    }
    pub fn has_some(&self) -> bool {
        match *self {
            Square { content: Some(_) } => true,
            _ => false,
        }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Square {
                    content: Some(ref p),
                } => format!("{}", p),
                Square { content: None } => format!("."),
            }
        )
    }
}

#[test]
fn helper_square_functions() {
    assert!(Square::white_king().has_white());
    assert!(Square::white_queen().has_white());
    assert!(Square::white_rook().has_white());
    assert!(Square::white_bishop().has_white());
    assert!(Square::white_knight().has_white());
    assert!(Square::white_pawn().has_white());

    assert!(Square::black_king().has_black());
    assert!(Square::black_queen().has_black());
    assert!(Square::black_rook().has_black());
    assert!(Square::black_bishop().has_black());
    assert!(Square::black_knight().has_black());
    assert!(Square::black_pawn().has_black());

    assert!(!Square::empty().has_white());
    assert!(!Square::empty().has_black());

    assert!(Square::empty().has_none());
    assert!(!Square::black_pawn().has_none());
    assert!(!Square::white_pawn().has_none());
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MoveType {
    Normal,
    LongCastling,
    ShortCastling,
    Promotion(PieceType),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub tipo: MoveType,
}

impl Move {
    pub fn new(from: Position, to: Position, movetype: MoveType) -> Move {
        Move {
            from: from,
            to: to,
            tipo: movetype,
        }
    }
    pub fn is_promotion(&self) -> bool {
        match self.tipo {
            MoveType::Promotion(_) => true,
            _ => false,
        }
    }
    pub fn from_string(s: &str) -> Result<Move, &'static str> {
        if s == "O-O" {
            Ok(Move {
                from: Position::new(0, 0),
                to: Position::new(0, 0),
                tipo: MoveType::ShortCastling,
            })
        } else if s == "O-O-O" {
            Ok(Move {
                from: Position::new(0, 0),
                to: Position::new(0, 0),
                tipo: MoveType::LongCastling,
            })
        } else if s.len() == 4 {
            let from_x: char = s.chars().nth(0).ok_or("Not found")?;
            let from_y: char = s.chars().nth(1).ok_or("Not found")?;
            let to_x: char = s.chars().nth(2).ok_or("Not found")?;
            let to_y: char = s.chars().nth(3).ok_or("Not found")?;
            Ok(Move {
                from: Position::from_chars(from_x, from_y)?,
                to: Position::from_chars(to_x, to_y)?,
                tipo: MoveType::Normal,
            })
        } else if s.len() == 5 {
            let from_x: char = s.chars().nth(0).ok_or("Nojt found")?;
            let from_y: char = s.chars().nth(1).ok_or("Not found")?;
            let to_x: char = s.chars().nth(2).ok_or("Not found")?;
            let to_y: char = s.chars().nth(3).ok_or("Not found")?;
            let prom: char = s.chars().nth(4).ok_or("Not found")?;
            Ok(Move {
                from: Position::from_chars(from_x, from_y)?,
                to: Position::from_chars(to_x, to_y)?,
                tipo: MoveType::Promotion(PieceType::from_char(prom)?),
            })
        } else {
            Err("Not a valid string")
        }
    }
    pub fn safe_from_string(s: &str) -> Move {
        Move::from_string(s).unwrap()
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.tipo {
            MoveType::Normal => write!(f, "{}{}", self.from, self.to),
            MoveType::LongCastling => write!(f, "O-O-O"),
            MoveType::ShortCastling => write!(f, "O-O"),
            MoveType::Promotion(ref pt) => write!(f, "{}{}{}", self.from, self.to, pt),
        }
    }
}

#[test]
fn move_from_string() {
    assert_eq!(
        Move::safe_from_string("a1a1"),
        Move::new(Position::new(2, 2), Position::new(2, 2), MoveType::Normal)
    );
    assert!(Move::from_string("z3a4").is_err());
    assert!(Move::from_string("e9a2").is_err());
    assert!(Move::from_string("aaaa").is_err());
    assert!(Move::from_string("aaa").is_err());
    assert!(Move::from_string("a").is_err());
}

#[derive(Debug, Clone, Copy)]
pub struct ValuedMove {
    pub mov: Move,
    pub value: BoardValue,
}

use self::BoardValue::Invalid;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum BoardValue {
    WonBlack,
    Value(i32),
    WonWhite,
    Invalid,
}

impl BoardValue {
    pub fn is_valid(&self) -> bool {
        match *self {
            Invalid => false,
            _ => true,
        }
    }
}

#[test]
fn test_cmp() {
    assert!(BoardValue::WonWhite > BoardValue::Value(200));
    assert!(BoardValue::WonWhite > BoardValue::WonBlack);
    assert!(BoardValue::WonBlack < BoardValue::Value(-100));
}

impl ValuedMove {
    pub fn from_move(mov: Move) -> ValuedMove {
        ValuedMove {
            mov: mov,
            value: Invalid,
        }
    }
    pub fn new(from: Position, to: Position, movetype: MoveType) -> ValuedMove {
        ValuedMove {
            mov: Move {
                from: from,
                to: to,
                tipo: movetype,
            },
            value: Invalid,
        }
    }
    pub fn invalid() -> ValuedMove {
        ValuedMove {
            mov: Move {
                from: Position::safe_from_chars('a', '1'),
                to: Position::safe_from_chars('a', '2'),
                tipo: MoveType::Normal,
            },
            value: Invalid,
        }
    }
    pub fn from_value(value: BoardValue) -> ValuedMove {
        ValuedMove {
            mov: Move {
                from: Position::safe_from_chars('a', '1'),
                to: Position::safe_from_chars('a', '2'),
                tipo: MoveType::Normal,
            },
            value: value,
        }
    }
}

impl fmt::Display for ValuedMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({:?})", self.mov, self.value)
    }
}

impl Ord for ValuedMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
impl PartialOrd for ValuedMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl PartialEq for ValuedMove {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for ValuedMove {}

#[test]
fn test_valuedmove_partial_ord() {
    let mut first = ValuedMove::invalid();
    let mut second = ValuedMove::invalid();
    assert_eq!(first, second);
    first.value = BoardValue::WonWhite;
    second.value = BoardValue::Value(10);
    assert!(first > second);
    first.value = BoardValue::Value(1);
    second.value = BoardValue::WonBlack;
    assert!(first > second);
}

pub type X = i8;
pub type Y = i8;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: X,
    pub y: Y,
}

use self::Direction::{Down, DownLeft, DownRight, Left, Right, Up, UpLeft, UpRight};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Direction {
    pub fn to_int(&self) -> usize {
        match *self {
            Up => 0,
            Down => 1,
            Left => 2,
            Right => 3,
            UpRight => 4,
            UpLeft => 5,
            DownRight => 6,
            DownLeft => 7,
        }
    }
}

impl Position {
    pub fn new(x: X, y: Y) -> Position {
        Position { x: x, y: y }
    }
    pub fn from_chars(x: char, y: char) -> Result<Position, &'static str> {
        Ok(Position {
            x: match x {
                c @ 'a'..='h' => Position::ch2x(c),
                _ => return Err("Bad letter"),
            },
            y: match y {
                c @ '1'..='8' => Position::ch2y(c),
                _ => return Err("Bad Number"),
            },
        })
    }
    pub fn safe_from_chars(x: char, y: char) -> Position {
        Position::from_chars(x, y).unwrap()
    }
    pub fn ch2y(y: char) -> Y {
        match y {
            '1' => 2,
            '2' => 3,
            '3' => 4,
            '4' => 5,
            '5' => 6,
            '6' => 7,
            '7' => 8,
            '8' => 9,
            _ => unreachable!(),
        }
    }
    pub fn ch2x(x: char) -> X {
        match x {
            'a' => 2,
            'b' => 3,
            'c' => 4,
            'd' => 5,
            'e' => 6,
            'f' => 7,
            'g' => 8,
            'h' => 9,
            _ => unreachable!(),
        }
    }
    pub fn go(&self, dir: Direction) -> Position {
        match dir {
            Up => self.up(),
            Down => self.down(),
            Right => self.right(),
            Left => self.left(),
            UpRight => self.up().right(),
            UpLeft => self.up().left(),
            DownRight => self.down().right(),
            DownLeft => self.down().left(),
        }
    }
    pub fn up(&self) -> Position {
        Position::new(self.x, self.y + 1)
    }
    pub fn down(&self) -> Position {
        Position::new(self.x, self.y - 1)
    }
    pub fn right(&self) -> Position {
        Position::new(self.x + 1, self.y)
    }
    pub fn left(&self) -> Position {
        Position::new(self.x - 1, self.y)
    }
    pub fn all() -> AllPositionsIterator {
        AllPositionsIterator {
            curr: Position::new(0, 0),
        }
    }
    pub fn iter_to(&self, dir: Direction) -> PositionIterator {
        PositionIterator {
            curr: *self,
            dir: dir,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.x {
                2 => 'a',
                3 => 'b',
                4 => 'c',
                5 => 'd',
                6 => 'e',
                7 => 'f',
                8 => 'g',
                9 => 'h',
                _ => 'E',
            },
            match self.y {
                2 => '1',
                3 => '2',
                4 => '3',
                5 => '4',
                6 => '5',
                7 => '6',
                8 => '7',
                9 => '8',
                _ => 'E',
            }
        )
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PositionDelta {
    pub x: X,
    pub y: Y,
}

impl PositionDelta {
    pub fn new(x: X, y: Y) -> PositionDelta {
        PositionDelta { x: x, y: y }
    }
    pub fn empty() -> PositionDelta {
        PositionDelta { x: 0, y: 0 }
    }
    pub fn from_dir(dir: Direction) -> PositionDelta {
        match dir {
            Up => PositionDelta::new(0, 1),
            Down => PositionDelta::new(0, -1),
            Right => PositionDelta::new(1, 0),
            Left => PositionDelta::new(-1, 0),

            UpRight => PositionDelta::new(1, 1),
            UpLeft => PositionDelta::new(-1, 1),
            DownRight => PositionDelta::new(1, -1),
            DownLeft => PositionDelta::new(-1, -1),
        }
    }
    pub fn from_dirs(dirs: &[Direction]) -> PositionDelta {
        dirs.iter().fold(PositionDelta::empty(), |pd, dir| {
            pd + PositionDelta::from_dir(*dir)
        })
    }
    pub fn apply_dir(&self, dir: Direction) -> PositionDelta {
        *self + PositionDelta::from_dir(dir)
    }
    pub fn apply_dirs(&self, dirs: &[Direction]) -> PositionDelta {
        *self + PositionDelta::from_dirs(dirs)
    }
    pub fn up(&self) -> PositionDelta {
        PositionDelta {
            y: self.y + 1,
            ..*self
        }
    }
    pub fn down(&self) -> PositionDelta {
        PositionDelta {
            y: self.y - 1,
            ..*self
        }
    }
    pub fn right(&self) -> PositionDelta {
        PositionDelta {
            x: self.x + 1,
            ..*self
        }
    }
    pub fn left(&self) -> PositionDelta {
        PositionDelta {
            x: self.x - 1,
            ..*self
        }
    }
}

impl Add<PositionDelta> for PositionDelta {
    type Output = PositionDelta;
    fn add(self, rhs: PositionDelta) -> Self::Output {
        PositionDelta {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<PositionDelta> for Position {
    type Output = Position;
    fn add(self, rhs: PositionDelta) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Neg for PositionDelta {
    type Output = PositionDelta;
    fn neg(self) -> Self::Output {
        PositionDelta {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[test]
fn test_add_position_delta() {
    assert_eq!(
        Position::safe_from_chars('a', '1') + PositionDelta::from_dirs(&[Up, Up]),
        Position::safe_from_chars('a', '3')
    );

    assert_eq!(
        Position::safe_from_chars('a', '1') + PositionDelta::from_dir(Right),
        Position::safe_from_chars('b', '1')
    );
}

pub struct AllPositionsIterator {
    pub curr: Position,
}

impl Iterator for AllPositionsIterator {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            Position { x: 0, y: 0 } => {
                self.curr.x = 2;
                self.curr.y = 2;
                Some(self.curr)
            }
            Position { x: 9, y: 9 } => None,
            Position { x: 9, y: _ } => {
                self.curr.y += 1;
                self.curr.x = 2;
                Some(self.curr)
            }
            Position { x: _, y: _ } => {
                self.curr.x += 1;
                Some(self.curr)
            }
        }
    }
}

pub struct PositionIterator {
    pub curr: Position,
    pub dir: Direction,
}

impl PositionIterator {
    pub fn new(pos: Position, dir: Direction) -> PositionIterator {
        PositionIterator {
            curr: pos,
            dir: dir,
        }
    }
}

impl Iterator for PositionIterator {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        self.curr = self.curr.go(self.dir);
        Some(self.curr)
    }
}

#[test]
fn test_positions() {
    assert_eq!(Position::all().count(), 64);
    assert_eq!(
        Position::safe_from_chars('h', '8'),
        Position::all().last().unwrap()
    );
    assert_eq!(
        Position::safe_from_chars('a', '1'),
        Position::all().nth(0).unwrap()
    );
    assert_eq!(
        Position::safe_from_chars('e', '1'),
        Position::all().nth(4).unwrap()
    );
    assert_eq!(
        Position::safe_from_chars('a', '2'),
        Position::all().nth(8).unwrap()
    );
    assert_eq!(
        Position::safe_from_chars('a', '5'),
        Position::all().nth(32).unwrap()
    );
    assert_eq!(
        Position::safe_from_chars('h', '8'),
        Position::all().nth(63).unwrap()
    );
}

#[test]
fn position_from_string() {
    assert_eq!(Position { x: 2, y: 2 }, Position::safe_from_chars('a', '1'));
    assert!(Position::from_chars('7', '1').is_err());
}

#[test]
fn move_position() {
    assert_eq!(
        Position::safe_from_chars('e', '2')
            .up()
            .down()
            .left()
            .right(),
        Position::safe_from_chars('e', '2')
    );

    assert_eq!(
        Position::safe_from_chars('e', '2').up(),
        Position::safe_from_chars('e', '3')
    );
    assert_eq!(
        Position::safe_from_chars('e', '2').down(),
        Position::safe_from_chars('e', '1')
    );
    assert_eq!(
        Position::safe_from_chars('e', '2').right(),
        Position::safe_from_chars('f', '2')
    );
    assert_eq!(
        Position::safe_from_chars('e', '2').left(),
        Position::safe_from_chars('d', '2')
    );

    assert_eq!(
        Position::safe_from_chars('e', '2').go(Up),
        Position::safe_from_chars('e', '3')
    );
    assert_eq!(
        Position::safe_from_chars('e', '2').go(Down),
        Position::safe_from_chars('e', '1')
    );
    assert_eq!(
        Position::safe_from_chars('e', '2').go(Right),
        Position::safe_from_chars('f', '2')
    );
    assert_eq!(
        Position::safe_from_chars('e', '2').go(Left),
        Position::safe_from_chars('d', '2')
    );
}
