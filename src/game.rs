#![allow(dead_code)]
use std::fmt;
use std::cmp;
use std::collections::{LinkedList};
use std::cmp::{Ord, Eq, PartialOrd, Ordering};
use std::ops::Not;

use std::ops::Add;

extern crate bit_vec;

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
                [None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None],
                [ // 1
                    None, None,
                    Some(Square::white_rook()),
                    Some(Square::white_knight()),
                    Some(Square::white_bishop()),
                    Some(Square::white_queen()),
                    Some(Square::white_king()),
                    Some(Square::white_bishop()),
                    Some(Square::white_knight()),
                    Some(Square::white_rook()),
                    None, None,
                ],
                [ // 2
                    None, None,
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    Some(Square::white_pawn()),
                    None, None,
                ],
                [ // 3
                    None, None,
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    None, None,
                ],
                [ // 4
                    None, None,
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    None, None,
                ],
                [ // 5
                    None, None,
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    None, None,
                ],
                [ // 6
                    None, None,
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    Some(Square::empty()),
                    None, None,
                ],
                [ // 7
                    None, None,
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    Some(Square::black_pawn()),
                    None, None,
                ],
                [ // 8
                    None, None,
                    Some(Square::black_rook()),
                    Some(Square::black_knight()),
                    Some(Square::black_bishop()),
                    Some(Square::black_queen()),
                    Some(Square::black_king()),
                    Some(Square::black_bishop()),
                    Some(Square::black_knight()),
                    Some(Square::black_rook()),
                    None, None,
                ],

                [None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None],
            ],
            turn: White,
        }
    }
    pub fn show(&self) {
        println!("{}", self)
    }
    pub fn get_raw_square(&self, pos: Position) -> Option<Square> {
        self.board[pos.y][pos.x]
    }
    pub fn get_square(&self, pos: Position) -> Square {
        self.board[pos.y][pos.x].unwrap()
    }
    pub fn get_piece(&self, pos: Position) -> Option<Piece> {
        self.get_square(pos).content
    }
    pub fn set_square(&mut self, pos: Position, piece: Option<Piece>) -> Result<(), &'static str> {
        Ok(self.board[pos.y][pos.x] = Some(Square::new(piece)))
    }
    pub fn is_square(&self, pos: Position) -> bool {
        self.board[pos.y][pos.x].is_some()
    }
    pub fn get_to_by(&self, mov: &Move, dir: Direction) -> bool {
        let mut p: Position = mov.from;
        loop {
            p = p.go(dir);
            if p == mov.to {
                return true
            };
            match self.get_raw_square(p) {
                Some(Square { content: None }) => {
                    continue
                },
                _ => return false,
            }
        };
    }
    pub fn make_move(&mut self, m: &Move) -> Result<(), &'static str> {
        match m.tipo {
            MoveType::Normal => {
                try!(self.is_valid_normal_move(m));
                try!(self.raw_make_move(m));
                Ok(())
            },
            MoveType::Promotion (pt) => {
                let col = self.turn;
                try!(self.is_valid_normal_move(m));
                try!(self.raw_make_move(m)); // BEWARE: This changes the color
                try!(self.set_square(m.to, Some(Piece::new(col, pt))));
                Ok(())
            },
            MoveType::LongCastling => { match self.turn {
                    White => {
                        let king_mov = Move::new(Position::safe_from_chars('e', '1'),
                                Position::safe_from_chars('c', '1'), MoveType::Normal);
                        let rook_mov = Move::new(Position::safe_from_chars('a', '1'),
                                Position::safe_from_chars('d', '1'), MoveType::Normal);

                        if self.get_square(rook_mov.from) == Square::white_rook() &&
                                self.get_square(king_mov.from) == Square::white_king() &&
                                self.get_to_by(&Move::new(king_mov.from, rook_mov.from, MoveType::Normal), |p| p.left()) {
                            try!(self.raw_move(&rook_mov));
                            try!(self.raw_make_move(&king_mov));
                            Ok(())
                        } else {
                            Err("Bad long casting")
                        }
                    },
                    Black => {
                        let king_mov = Move::new(Position::safe_from_chars('e', '8'),
                                Position::safe_from_chars('c', '8'), MoveType::Normal);
                        let rook_mov = Move::new(Position::safe_from_chars('a', '8'),
                                Position::safe_from_chars('d', '8'), MoveType::Normal);

                        if self.get_square(rook_mov.from) == Square::white_rook() &&
                                self.get_square(king_mov.from) == Square::white_king() &&
                                self.get_to_by(&Move::new(king_mov.from, rook_mov.from, MoveType::Normal), |p| p.left()) {
                            try!(self.raw_move(&rook_mov));
                            try!(self.raw_make_move(&king_mov));
                            Ok(())
                        } else {
                            Err("Bad long casting")
                        }
                    },
                }
            },
            MoveType::ShortCastling => match self.turn {
                White => {
                    let king_mov = Move::new(Position::safe_from_chars('e', '1'),
                            Position::safe_from_chars('g', '1'), MoveType::Normal);
                    let rook_mov = Move::new(Position::safe_from_chars('h', '1'),
                            Position::safe_from_chars('f', '1'), MoveType::Normal);

                    if self.get_square(rook_mov.from) == Square::white_rook() &&
                            self.get_square(king_mov.from) == Square::white_king() &&
                            self.get_to_by(&Move::new(king_mov.from, rook_mov.from, MoveType::Normal), |p| p.right()) {
                        try!(self.raw_move(&rook_mov));
                        try!(self.raw_make_move(&king_mov));
                        Ok(())
                    } else {
                        Err("Bad short casting")
                    }
                },
                Black => {
                    let king_mov = Move::new(Position::safe_from_chars('e', '8'),
                            Position::safe_from_chars('g', '8'), MoveType::Normal);
                    let rook_mov = Move::new(Position::safe_from_chars('h', '8'),
                            Position::safe_from_chars('f', '8'), MoveType::Normal);

                    if self.get_square(rook_mov.from) == Square::black_rook() &&
                            self.get_square(king_mov.from) == Square::black_king() &&
                            self.get_to_by(&Move::new(king_mov.from, rook_mov.from, MoveType::Normal), |p| p.right()) {
                        try!(self.raw_move(&rook_mov));
                        try!(self.raw_make_move(&king_mov));
                        Ok(())
                    } else {
                        Err("Bad short casting")
                    }
                },
            },
        }
    }
    pub fn is_valid_normal_move(&self, m: &Move) -> Result<(), &'static str> {
        // Option<{ content: Option<{ pieceType: PieceType, color: Color }> }>
        println!("Making move {}...", m);
        match (self.board[m.from.y][m.from.x], self.board[m.to.y][m.to.x]) {
            (None, _)                       => Err("Not even a valid square"),
            (_, None)                       => Err("Not even a valid square"),
            (Some(Square {content: None}), _) => Err("Empty from square"),
            (Some(from_square), Some(to_square)) => {
                if from_square.get_piece().color != self.turn {
                    return Err("Wrong color")
                };
                match from_square.get_piece() {
                    piece!(color, King)   => {
                        if  (
                                m.from.up()==m.to ||
                                m.from.down()==m.to ||
                                m.from.left()==m.to ||
                                m.from.right()==m.to ||
                                m.from.up().right()==m.to ||
                                m.from.up().left()==m.to ||
                                m.from.down().right()==m.to ||
                                m.from.down().left()==m.to
                            ) &&
                                ! to_square.has_color(color)
                        {
                            Ok(())
                        } else {
                            Err("Bad King movement")
                        }
                    },
                    piece!(color, Queen) => {
                        if  (
                                self.get_to_by(m, |p| p.up().right()) ||
                                self.get_to_by(m, |p| p.up().left()) ||
                                self.get_to_by(m, |p| p.down().right()) ||
                                self.get_to_by(m, |p| p.down().left()) ||
                                self.get_to_by(m, |p| p.up()) ||
                                self.get_to_by(m, |p| p.down()) ||
                                self.get_to_by(m, |p| p.right()) ||
                                self.get_to_by(m, |p| p.left())
                            ) &&
                                ! to_square.has_color(color)
                            {
                            Ok(())
                        } else {
                            Err("Bad Queen movement")
                        }
                    },
                    piece!(color, Rook) => {
                        if  (
                                self.get_to_by(m, |p| p.up()) ||
                                self.get_to_by(m, |p| p.down()) ||
                                self.get_to_by(m, |p| p.right()) ||
                                self.get_to_by(m, |p| p.left())
                            ) &&
                                ! to_square.has_color(color)
                            {
                            Ok(())
                        } else {
                            Err("Bad Rook movement")
                        }
                    },
                    piece!(color, Bishop) => {
                        if  (
                                self.get_to_by(m, |p| p.up().right()) ||
                                self.get_to_by(m, |p| p.up().left()) ||
                                self.get_to_by(m, |p| p.down().right()) ||
                                self.get_to_by(m, |p| p.down().left())
                            ) &&
                                ! to_square.has_color(color)
                            {
                            Ok(())
                        } else {
                            Err("Bad Bishop movement")
                        }
                    },
                    piece!(color, Knight) => {
                        if  (
                                m.from.up().up().right()==m.to ||
                                m.from.up().up().left()==m.to ||
                                m.from.down().down().right()==m.to ||
                                m.from.down().down().left()==m.to ||
                                m.from.right().right().up()==m.to ||
                                m.from.right().right().down()==m.to ||
                                m.from.left().left().up()==m.to ||
                                m.from.left().left().down()==m.to
                            ) &&
                                ! to_square.has_color(color)
                            {
                            Ok(())
                        } else {
                            Err("Bad Knight movement")
                        }
                    },
                    piece!(White, Pawn)   => {
                        if m.from.y==Position::ch2y('7') && ! m.is_promotion() {
                            Err("You must promote that pawn")
                        } else if (
                                m.from.y==Position::ch2y('2') &&
                                m.from.up().up()==m.to &&
                                to_square.has_none() &&
                                self.get_to_by(m, |p| p.up())
                            ) || (
                                m.from.up()==m.to &&
                                to_square.has_none()
                            ) || (
                                (
                                    m.from.up().left()==m.to ||
                                    m.from.up().right()==m.to
                                ) && to_square.has_black()
                            ) {
                            Ok(())
                        } else {
                            Err("Bad pawn movement")
                        }
                    },
                    piece!(Black, Pawn)   => {
                        if m.from.y==Position::ch2y('2') && ! m.is_promotion() {
                            Err("You must promote that pawn")
                        } else if (
                                m.from.y==Position::ch2y('7') &&
                                m.from.down().down()==m.to &&
                                to_square.has_none() &&
                                self.get_to_by(m, |p| p.down())
                            ) || (
                                m.from.down()==m.to &&
                                to_square.has_none()
                            ) || (
                                (
                                    m.from.down().left()==m.to ||
                                    m.from.down().right()==m.to
                                ) && to_square.has_white()
                            ) {
                            Ok(())
                        } else {
                            Err("Bad pawn movement")
                        }
                    },
                }
            },
        }
    }
    pub fn raw_make_move(&mut self, m: &Move) -> Result<(), &'static str> {
        try!(self.raw_move(m));
        self.turn = !self.turn;
        Ok(())
    }
    pub fn raw_move(&mut self, m: &Move) -> Result<(), &'static str> {
        let from_piece = self.get_piece(m.from);
        try!(self.set_square(m.to,   from_piece));
        try!(self.set_square(m.from, None));
        Ok(())
    }
    pub fn get_all_valid_moves(&self) -> LinkedList<ValuedMove> {
        let mut moves = LinkedList::new();
        for from_pos in Position::all() {
            if !  self.get_square(from_pos).has_color(self.turn) {
                continue
            };
            moves.append(&mut self.get_valid_moves(from_pos));
        };
        moves
    }
    pub fn get_valid_moves(&self, from_pos: Position) -> LinkedList<ValuedMove> {
        let mut moves = LinkedList::new();
        let piece = match self.get_raw_square(from_pos) {
            Some(Square { content: Some(piece)}) => piece,
            _ => return moves,
        };
        match piece {
            piece!(color, King)   => {
                for to_pos in [
                        from_pos.up(),
                        from_pos.down(),
                        from_pos.right(),
                        from_pos.left(),
                        from_pos.up()  .right(),
                        from_pos.up()  .left(),
                        from_pos.down().right(),
                        from_pos.down().left(),
                                            ].iter() {
                    if let Some(to_square) = self.get_raw_square(*to_pos) {
                        if ! to_square.has_color(color) {
                            moves.push_back(ValuedMove::new(from_pos, *to_pos, MoveType::Normal));
                        }
                    }
                }
            },
            piece!(color, Queen)  => {
                for dir in [
                        Direction::Up, Direction::Down,
                        Direction::Left, Direction::Right,
                        Direction::UpRight, Direction::UpLeft,
                        Direction::DownRight, Direction::DownLeft
                        ].iter(){
                    for to_pos in from_pos.iter_to(*dir){
                        match self.get_raw_square(to_pos) {
                            Some(Square {content: Some(piece!(to_color, _))}) => {
                                if color != to_color {
                                    moves.push_back(ValuedMove::new(from_pos, to_pos, MoveType::Normal));
                                }
                                break
                            },
                            Some(Square {content: None }) => {
                                    moves.push_back(ValuedMove::new(from_pos, to_pos, MoveType::Normal));
                            },
                            _ => break,
                        }
                    }
                }
            },
            piece!(color, Rook)   => {
                for dir in [Direction::Up, Direction::Down, Direction::Left, Direction::Right].iter(){
                    for to_pos in from_pos.iter_to(*dir){
                        match self.get_raw_square(to_pos) {
                            Some(Square {content: Some(piece!(to_color, _))}) => {
                                if color != to_color {
                                    moves.push_back(ValuedMove::new(from_pos, to_pos, MoveType::Normal));
                                }
                                break
                            },
                            Some(Square {content: None }) => {
                                    moves.push_back(ValuedMove::new(from_pos, to_pos, MoveType::Normal));
                            },
                            _ => break,
                        }
                    }
                }
            },
            piece!(color, Bishop) => {
                for dir in [
                        Direction::UpRight, Direction::UpLeft,
                        Direction::DownRight, Direction::DownLeft].iter(){
                    for to_pos in from_pos.iter_to(*dir){
                        match self.get_raw_square(to_pos) {
                            Some(Square {content: Some(piece!(to_color, _))}) => {
                                if color != to_color {
                                    moves.push_back(ValuedMove::new(from_pos, to_pos, MoveType::Normal));
                                }
                                break
                            },
                            Some(Square {content: None }) => {
                                    moves.push_back(ValuedMove::new(from_pos, to_pos, MoveType::Normal));
                            },
                            _ => break,
                        }
                    }
                }
            },
            piece!(color, Knight) => {
                for to_pos in [ from_pos.up()    .up()    .right(),
                                from_pos.up()    .up()    .left(),
                                from_pos.down()  .down()  .right(),
                                from_pos.down()  .down()  .left(),
                                from_pos.right() .right() .up(),
                                from_pos.right() .right() .down(),
                                from_pos.left()  .left()  .up(),
                                from_pos.left()  .left()  .down()].iter() {
                    if let Some(to_square) = self.get_raw_square(*to_pos) {
                        if ! to_square.has_color(color) {
                            moves.push_back(ValuedMove::new(from_pos, *to_pos, MoveType::Normal));
                        }
                    }
                }
            },
            piece!(White, Pawn)   => {
                let promotion_y = Position::ch2y('7');
                let long_move_y = Position::ch2y('2');
                for to_pos in [ from_pos.up().left(),
                                from_pos.up().right(),]
                                                .iter() {
                    if let Some(to_square) = self.get_raw_square(*to_pos) {
                        if to_square.has_black() {
                            if from_pos.y==promotion_y {
                                for promotion_piece in [Queen, Rook, Bishop, Knight].iter() {
                                    moves.push_back(ValuedMove::new(from_pos, *to_pos,
                                        MoveType::Promotion(*promotion_piece)));
                                }
                            } else {
                                moves.push_back(ValuedMove::new(from_pos, *to_pos, MoveType::Normal))
                            }
                        }
                    }
                }
                let to_pos = from_pos.up();
                if let Some(to_square) = self.get_raw_square(to_pos) {
                    if to_square.has_none() {
                        if from_pos.y==promotion_y {
                            for promotion_piece in [Queen, Rook, Bishop, Knight].iter() {
                                moves.push_back(ValuedMove::new(from_pos, to_pos,
                                    MoveType::Promotion(*promotion_piece)));
                            }
                        } else {
                            moves.push_back(ValuedMove::new(from_pos, to_pos, MoveType::Normal))
                        }
                        let to_pos = to_pos.up(); // TODO Much nest
                        if let Some(to_square) = self.get_raw_square(to_pos) {
                            if from_pos.y == long_move_y && to_square.has_none() {
                                moves.push_back(ValuedMove::new(from_pos, to_pos, MoveType::Normal));
                            }
                        }
                    }
                }
            },
            piece!(Black, Pawn)   => {
                let promotion_y = Position::ch2y('2');
                let long_move_y = Position::ch2y('7');
                for to_pos in [ from_pos.down().left(),
                                from_pos.down().right(),]
                                                .iter() {
                    if let Some(to_square) = self.get_raw_square(*to_pos) {
                        if to_square.has_white() {
                            if from_pos.y==promotion_y {
                                for promotion_piece in [Queen, Rook, Bishop, Knight].iter() {
                                    moves.push_back(ValuedMove::new(from_pos, *to_pos,
                                        MoveType::Promotion(*promotion_piece)));
                                }
                            } else {
                                moves.push_back(ValuedMove::new(from_pos, *to_pos, MoveType::Normal))
                            }
                        }
                    }
                }
                let to_pos = from_pos.down();
                if let Some(to_square) = self.get_raw_square(to_pos) {
                    if to_square.has_none() {
                        if from_pos.y==promotion_y {
                            for promotion_piece in [Queen, Rook, Bishop, Knight].iter() {
                                moves.push_back(ValuedMove::new(from_pos, to_pos,
                                    MoveType::Promotion(*promotion_piece)));
                            }
                        } else {
                            moves.push_back(ValuedMove::new(from_pos, to_pos, MoveType::Normal))
                        }
                        let to_pos = to_pos.down(); // TODO Much nest
                        if let Some(to_square) = self.get_raw_square(to_pos) {
                            if from_pos.y == long_move_y && to_square.has_none() {
                                moves.push_back(ValuedMove::new(from_pos, to_pos, MoveType::Normal));
                            }
                        }
                    }
                }
            },
        };
        moves
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} Plays\n", self.turn));
        try!(write!(f, "  +-----------------+\n"));
        for r in (2..10).rev() {
            try!(write!(f, "{} |", r-1));
            for c in 2..10 {
                match self.board[r][c] {
                    Some(ref p) => try!(write!(f, " {}", p)),
                    None => try!(write!(f, "")),
                };
            }
            try!(write!(f, " |\n"));
        };
        try!(write!(f, "  +-----------------+\n"));
        try!(write!(f, "    a b c d e f g h  \n"));
        Ok(())
    }
}

#[test]
fn test_helper_functions() {
    let mut game = Game::new();
    let pos = Position::safe_from_chars('e','2');
    assert!(!game.is_square(Position::new(0,0)));
    assert!(game.is_square(pos));
    assert_eq!(game.get_square(pos), Square::white_pawn());
    assert!(game.make_move(&Move::safe_from_string("e2e4")).is_ok());
    assert!(game.make_move(&Move::safe_from_string("e2d4")).is_err());
}
#[test]
fn test_game_1() {
    let mut game = Game::new();
    assert!(game.make_move(&Move::safe_from_string("e2e4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("e7e5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g1f3")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("b8c6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f1c4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f8c5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c2c3")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c5b6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d2d4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d8e7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("O-O")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g8f6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d4d5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c6b8")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c4d3")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d7d6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("b1d2")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("a7a6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d2c4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("b6a7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("a2a4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("O-O")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("b2b4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f6e8")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d1c2")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g7g6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c1h6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("e8g7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c4e3")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f7f6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("a1e1")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f8f7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g1h1")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("b8d7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g2g4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d7f8")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f1g1")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("a7e3")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f2e3")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c8d7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g1g3")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c7c6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d3c4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c6d5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c4d5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d7e6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("e1g1")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("a8c8")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f3h4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("e6d5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("e4d5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c8c7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("h4f5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g6f5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g4f5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("e7e8")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c2g2")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("e8d7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g3g7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f7g7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("h6g7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d7g7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g2c2")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f8g6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f5g6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("h7h6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c2f5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g7f8")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c3c4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g8g7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g1c1")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("b7b6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("e3e4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f8e7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f5f2")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c7b7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("h2h4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("a6a5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("h4h5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("a5b4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c1b1")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("b4b3")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("b1b3")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("e7d7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f2f5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d7e7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f5e6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("e7c7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("e6f7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c7f7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g6f7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("b7a7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("b3b6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("a7a4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("b6d6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("a4c4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d6f6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("g7f8")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d5d6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("c4e4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d6d7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("e4d4")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f6h6")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("f8f7")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("h6h8")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d4d5")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d7d8q")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("d5d8")).is_ok()); game.show();
    assert!(game.make_move(&Move::safe_from_string("h8d8")).is_ok()); game.show();
}

use self::Color::{White, Black};
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn get_sign(&self) -> i32{
        match *self {
            White =>  1,
            Black => -1,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            White => "White",
            Black => "Black",
        })
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

impl PieceType {
    pub fn from_char(c: char) -> Result<PieceType, &'static str> {
        match c {
            'r' => Ok(Rook),
            'n' => Ok(Knight),
            'b' => Ok(Bishop),
            'q' => Ok(Queen),
            'k' => Ok(King),
            'p' => Ok(Pawn),
             _  => Err("Not a valid piece type"),
        }
    }
    pub fn safe_from_char(c: char) -> PieceType {
        PieceType::from_char(c).unwrap()
    }
    pub fn get_value(&self) -> i32 {
        match *self {
            King   => 128,
            Queen  => 9,
            Rook   => 5,
            Bishop => 3,
            Knight => 3,
            Pawn   => 1,
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Rook => 'r',
            Knight => 'n',
            Bishop => 'b',
            Queen => 'q',
            King => 'k',
            Pawn => 'p'
        })
    }
}

#[test]
fn piecetype_from_char () {
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
        Piece {
            tipo: t,
            color: c,
        }
    }
    pub fn get_value(&self) -> i32 {
        self.color.get_sign() * self.tipo.get_value()
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            piece!(Black, Rook)   => 'R',
            piece!(Black, Knight) => 'N',
            piece!(Black, Bishop) => 'B',
            piece!(Black, Queen)  => 'Q',
            piece!(Black, King)   => 'K',
            piece!(Black, Pawn)   => 'P',

            piece!(White, Rook)   => 'r',
            piece!(White, Knight) => 'n',
            piece!(White, Bishop) => 'b',
            piece!(White, Queen)  => 'q',
            piece!(White, King)   => 'k',
            piece!(White, Pawn)   => 'p',
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Square {
    pub content: Option<Piece>
}

impl Square {
    pub fn new(p: Option<Piece>) -> Square {
        Square { content: p }
    }

    pub fn black_rook() -> Square {
        Square { content: Some(Piece { tipo: Rook, color: Black }) }
    }
    pub fn black_knight() -> Square {
        Square { content: Some(Piece { tipo: Knight, color: Black }) }
    }
    pub fn black_bishop() -> Square {
        Square { content: Some(Piece { tipo: Bishop, color: Black }) }
    }
    pub fn black_queen() -> Square {
        Square { content: Some(Piece { tipo: Queen, color: Black }) }
    }
    pub fn black_king() -> Square {
        Square { content: Some(Piece { tipo: King, color: Black }) }
    }
    pub fn black_pawn() -> Square {
        Square { content: Some(Piece { tipo: Pawn, color: Black }) }
    }

    pub fn white_rook() -> Square {
        Square { content: Some(Piece { tipo: Rook, color: White }) }
    }
    pub fn white_knight() -> Square {
        Square { content: Some(Piece { tipo: Knight, color: White }) }
    }
    pub fn white_bishop() -> Square {
        Square { content: Some(Piece { tipo: Bishop, color: White }) }
    }
    pub fn white_queen() -> Square {
        Square { content: Some(Piece { tipo: Queen, color: White }) }
    }
    pub fn white_king() -> Square {
        Square { content: Some(Piece { tipo: King, color: White }) }
    }
    pub fn white_pawn() -> Square {
        Square { content: Some(Piece { tipo: Pawn, color: White }) }
    }
    pub fn empty() -> Square {
        Square { content: None }
    }

    pub fn get_piece(&self) -> Piece {
        self.content.unwrap()
    }

    pub fn has_color(&self, color: Color) -> bool {
        match (color, *self) {
            (White, Square { content: Some(piece!(White, _)) }) => true,
            (Black, Square { content: Some(piece!(Black, _)) }) => true,
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
            Square { content: Some(piece!(White, _)) } => true,
            _ => false,
        }
    }
    pub fn has_black(&self) -> bool {
        match *self {
            Square { content: Some(piece! (Black, _)) } => true,
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
        write!(f, "{}", match *self {
            Square { content: Some(ref p) } => format!("{}", p),
            Square { content: None } => format!("."),
        })
    }
}

#[test]
fn helper_square_functions() {
    assert!(Square::white_king()   .has_white());
    assert!(Square::white_queen()  .has_white());
    assert!(Square::white_rook()   .has_white());
    assert!(Square::white_bishop() .has_white());
    assert!(Square::white_knight() .has_white());
    assert!(Square::white_pawn()   .has_white());

    assert!(Square::black_king()   .has_black());
    assert!(Square::black_queen()  .has_black());
    assert!(Square::black_rook()   .has_black());
    assert!(Square::black_bishop() .has_black());
    assert!(Square::black_knight() .has_black());
    assert!(Square::black_pawn()   .has_black());

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
    Promotion (PieceType),
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
            to:   to,
            tipo: movetype,
        }
    }
    pub fn is_promotion(&self) -> bool {
        match self.tipo {
            MoveType::Promotion (_) => true,
            _ => false,
        }
    }
    pub fn from_string(s: &str) -> Result<Move, &'static str> {
        if s == "O-O" {
            Ok(Move {
                from: Position::new(0,0),
                to: Position::new(0,0),
                tipo: MoveType::ShortCastling,
            })
        } else if s == "O-O-O" {
            Ok(Move {
                from: Position::new(0,0),
                to: Position::new(0,0),
                tipo: MoveType::LongCastling,
            })
        } else if s.len() == 4 {
            let from_x: char = try!(s.chars().nth(0).ok_or("Not found"));
            let from_y: char = try!(s.chars().nth(1).ok_or("Not found"));
            let to_x: char = try!(s.chars().nth(2).ok_or("Not found"));
            let to_y: char = try!(s.chars().nth(3).ok_or("Not found"));
            Ok( Move {
                from: try!(Position::from_chars(from_x, from_y)),
                to: try!(Position::from_chars(to_x, to_y)),
                tipo: MoveType::Normal,
            })
        } else if s.len() == 5 {
            let from_x: char = try!(s.chars().nth(0).ok_or("Nojt found"));
            let from_y: char = try!(s.chars().nth(1).ok_or("Not found"));
            let to_x: char = try!(s.chars().nth(2).ok_or("Not found"));
            let to_y: char = try!(s.chars().nth(3).ok_or("Not found"));
            let prom: char = try!(s.chars().nth(4).ok_or("Not found"));
            Ok( Move {
                from: try!(Position::from_chars(from_x, from_y)),
                to: try!(Position::from_chars(to_x, to_y)),
                tipo: MoveType::Promotion(try!(PieceType::from_char(prom))),
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
            MoveType::Promotion (ref pt) => write!(f, "{}{}{}", self.from, self.to, pt),
        }
    }
}

#[test]
fn move_from_string() {
    assert_eq!(Move::safe_from_string("a1a1"), Move::new(Position::new(2,2), Position::new(2,2), MoveType::Normal));
    assert!(Move::from_string("z3a4").is_err());
    assert!(Move::from_string("e9a2").is_err());
    assert!(Move::from_string("aaaa").is_err());
    assert!(Move::from_string("aaa").is_err());
    assert!(Move::from_string("a").is_err());
}

#[derive(Debug, Clone, Copy)]
pub struct ValuedMove {
    pub value: Option<i32>,
    pub mov: Move,
}

impl ValuedMove {
    pub fn is_valued(&self) -> bool {
        self.value.is_some()
    }
    pub fn from_move(mov: Move) -> ValuedMove {
        ValuedMove { mov: mov, value: None }
    }
    pub fn new(from: Position, to: Position, movetype: MoveType) -> ValuedMove {
        ValuedMove {
            mov: Move {
                from: from,
                to:   to,
                tipo: movetype,
            },
            value: None
        }
    }
    pub fn empty() -> ValuedMove {
        ValuedMove {
            mov: Move {
                from: Position::safe_from_chars('a','1'),
                to:   Position::safe_from_chars('a','2'),
                tipo: MoveType::Normal,
            },
            value: None
        }
    }
}

impl Ord for ValuedMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}
impl PartialOrd for ValuedMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.value, other.value) {
            (None, None) => None,
            (None, Some(_)) => Some(Ordering::Less),
            (Some(_), None) => Some(Ordering::Greater),
            (Some(me), Some(you)) => Some(me.cmp(&you)),
        }
    }
}
impl PartialEq for ValuedMove {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl Eq for ValuedMove { }

#[test]
fn test_valuedmove_partial_ord() {
    let mut big = ValuedMove::empty();
    let mut small = ValuedMove::empty();
    assert_eq!(big, small);
    big.value = Some(10);
    assert!(big > small);
    assert!(small < big);
    small.value = Some(1);
    assert!(big > small);
    assert!(small < big);
}

pub type X = usize;
pub type Y = usize;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: X,
    pub y: Y,
}

use self::Direction::{Up, Down, Left, Right, UpRight, UpLeft, DownRight, DownLeft};
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
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
            Direction::UpRight => 4,
            Direction::UpLeft => 5,
            Direction::DownRight => 6,
            Direction::DownLeft => 7,
        }
    }
}

impl Position {
    pub fn new(x: X, y: Y) -> Position {
        Position { x: x, y: y}
    }
    pub fn from_chars(x: char, y: char) -> Result<Position, &'static str> {
        Ok(Position {
            x: match x {
                c @ 'a' ... 'h' => Position::ch2x(c),
                 _  => return Err("Bad letter"),
            },
            y: match y {
                c @ '1' ... '8' => Position::ch2y(c),
                 _  => return Err("Bad Number"),
            },
        })
    }
    pub fn safe_from_chars(x: char, y: char) -> Position {
        Position::from_chars(x,y).unwrap()
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
             _  => unreachable!(),
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
             _  => unreachable!(),
        }
    }
    pub fn go(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up        => self.up(),
            Direction::Down      => self.down(),
            Direction::Right     => self.right(),
            Direction::Left      => self.left(),
            Direction::UpRight   => self.up().right(),
            Direction::UpLeft    => self.up().left(),
            Direction::DownRight => self.down().right(),
            Direction::DownLeft  => self.down().left(),
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
        AllPositionsIterator { curr: Position::new(0,0)}
    }
    pub fn iter_to(&self, dir: Direction) -> PositionIterator {
        PositionIterator { curr: *self, dir: dir}
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}{}",
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

pub struct AllPositionsIterator {
    pub curr: Position,
}

impl Iterator for AllPositionsIterator {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item>{
        match self.curr {
            Position { x: 0, y: 0 } => {
                self.curr.x = 2;
                self.curr.y = 2;
                Some(self.curr)
            },
            Position { x: 9, y: 9 } => {
                None
            },
            Position { x: 9, y: _ } => {
                self.curr.y += 1;
                self.curr.x = 2;
                Some(self.curr)
            },
            Position { x: _, y: _ } => {
                self.curr.x += 1;
                Some(self.curr)
            },
        }
    }
}

pub struct PositionIterator {
    pub curr: Position,
    pub dir: Direction,
}

impl PositionIterator {
    pub fn new(pos: Position, dir: Direction) -> PositionIterator {
        PositionIterator { curr: pos, dir: dir }
    }
}

impl Iterator for PositionIterator {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item>{
        self.curr = self.curr.go(self.dir);
        Some(self.curr)
    }
}

#[test]
fn test_positions() {
    assert_eq!(Position::all().count(), 64);
    assert_eq!(Position::safe_from_chars('h','8'), Position::all().last().unwrap());
    assert_eq!(Position::safe_from_chars('a','1'), Position::all().nth(0).unwrap());
    assert_eq!(Position::safe_from_chars('e','1'), Position::all().nth(4).unwrap());
    assert_eq!(Position::safe_from_chars('a','2'), Position::all().nth(8).unwrap());
    assert_eq!(Position::safe_from_chars('a','5'), Position::all().nth(32).unwrap());
    assert_eq!(Position::safe_from_chars('h','8'), Position::all().nth(63).unwrap());
}

#[test]
fn position_from_string() {
    assert_eq!(Position { x: 2, y: 2 }, Position::safe_from_chars('a', '1'));
    assert!(Position::from_chars('7', '1').is_err());
}

#[test]
fn move_position() {
    assert_eq!(Position::safe_from_chars('e','2').up().down().left().right(), Position::safe_from_chars('e','2'));

    assert_eq!(Position::safe_from_chars('e','2').up(), Position::safe_from_chars('e','3'));
    assert_eq!(Position::safe_from_chars('e','2').down(), Position::safe_from_chars('e','1'));
    assert_eq!(Position::safe_from_chars('e','2').right(), Position::safe_from_chars('f','2'));
    assert_eq!(Position::safe_from_chars('e','2').left(), Position::safe_from_chars('d','2'));

    assert_eq!(Position::safe_from_chars('e','2').go(Direction::Up), Position::safe_from_chars('e','3'));
    assert_eq!(Position::safe_from_chars('e','2').go(Direction::Down), Position::safe_from_chars('e','1'));
    assert_eq!(Position::safe_from_chars('e','2').go(Direction::Right), Position::safe_from_chars('f','2'));
    assert_eq!(Position::safe_from_chars('e','2').go(Direction::Left), Position::safe_from_chars('d','2'));
}
