use std::fmt;
use std::ops::Not;

type Board = [[Option<Square>; 12]; 12];

pub struct Game {
    board: Board,
    turn: Color,
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
    fn get_raw_square(&self, pos: Position) -> Option<Square> {
        self.board[pos.y][pos.x]
    }
    fn get_square(&self, pos: Position) -> Square {
        self.board[pos.y][pos.x].unwrap()
    }
    fn set_square(&mut self, pos: Position, piece: Option<Piece>) -> Result<(), &'static str> {
        Ok(self.board[pos.y][pos.x] = Some(Square::new(piece)))
    }
    fn is_square(&self, pos: Position) -> bool {
        self.board[pos.y][pos.x].is_some()
    }
    fn get_to_by<F>(&self, mov: &Move, f: F) -> bool where F: Fn(&Position) -> Position {
        let mut p: Position = mov.from;
        loop {
            p = f(&p);
            if p == mov.to {
                return true
            };
            match self.get_raw_square(p) {
                Some(Square { content: None }) => { println!("{} containts '{}'", p, self.get_square(p)); continue },
                _ => return false,
            }
        };
    }
    pub fn make_move(&mut self, m: &Move) -> Result<(), &'static str> {
        match m.tipo {
            MoveType::Normal => self.make_normal_move(m),
            MoveType::Promotion (pt) => {
                let col = self.turn;
                try!(self.make_normal_move(m));
                try!(self.set_square(m.to, Some(Piece::new(pt, col))));
                Ok(())
            },
            MoveType::LongCastling => {
                match self.turn {
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
    fn make_normal_move(&mut self, m: &Move) -> Result<(), &'static str> {
        // Option<{ content: Option<{ pieceType: PieceType, color: Color }> }>
        println!("Making move {}...", m);
        match (self.board[m.from.y][m.from.x], self.board[m.to.y][m.to.x]) {
            (None, _)                       => Err("Not even a valid square"),
            (_, None)                       => Err("Not even a valid square"),
            (Some(Square {content: None}), _) => Err("Empty from square"),
            (Some(from_square), Some(to_square)) => {
                if from_square.content.unwrap().color != self.turn {
                    return Err("Wrong color")
                };
                match self.turn {
                    White => match from_square.content.unwrap().tipo {
                        King   => {
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
                                    ! to_square.has_white()
                            {
                                self.raw_make_move(m)
                            } else {
                                Err("Bad King movement")
                            }
                        },
                        Queen  => {
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
                                    ! to_square.has_white()
                                {
                                self.raw_make_move(m)
                            } else {
                                Err("Bad Queen movement")
                            }
                        },
                        Rook   => {
                            if  (
                                    self.get_to_by(m, |p| p.up()) ||
                                    self.get_to_by(m, |p| p.down()) ||
                                    self.get_to_by(m, |p| p.right()) ||
                                    self.get_to_by(m, |p| p.left())
                                ) &&
                                    ! to_square.has_white()
                                {
                                self.raw_make_move(m)
                            } else {
                                Err("Bad Rook movement")
                            }
                        },
                        Bishop => {
                            if  (
                                    self.get_to_by(m, |p| p.up().right()) ||
                                    self.get_to_by(m, |p| p.up().left()) ||
                                    self.get_to_by(m, |p| p.down().right()) ||
                                    self.get_to_by(m, |p| p.down().left())
                                ) &&
                                    ! to_square.has_white()
                                {
                                self.raw_make_move(m)
                            } else {
                                Err("Bad Bishop movement")
                            }
                        },
                        Knight => {
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
                                    ! to_square.has_white()
                                {
                                self.raw_make_move(m)
                            } else {
                                Err("Bad Knight movement")
                            }
                        },
                        Pawn   => {
                            if (
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
                                self.raw_make_move(m)
                            } else {
                                Err("Bad pawn movement")
                            }
                        },
                    },
                    Black => match from_square.content.unwrap().tipo {
                        King   => {
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
                                    ! to_square.has_black()
                            {
                                self.raw_make_move(m)
                            } else {
                                Err("Bad King movement")
                            }
                        },
                        Queen  => {
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
                                    ! to_square.has_black()
                                {
                                self.raw_make_move(m)
                            } else {
                                Err("Bad Queen movement")
                            }
                        },
                        Rook   => {
                            if  (
                                    self.get_to_by(m, |p| p.up()) ||
                                    self.get_to_by(m, |p| p.down()) ||
                                    self.get_to_by(m, |p| p.right()) ||
                                    self.get_to_by(m, |p| p.left())
                                ) &&
                                    ! to_square.has_black()
                                {
                                self.raw_make_move(m)
                            } else {
                                Err("Bad Rook movement")
                            }
                        },
                        Bishop => {
                            if  (
                                    self.get_to_by(m, |p| p.up().right()) ||
                                    self.get_to_by(m, |p| p.up().left()) ||
                                    self.get_to_by(m, |p| p.down().right()) ||
                                    self.get_to_by(m, |p| p.down().left())
                                ) &&
                                    ! to_square.has_black()
                                {
                                self.raw_make_move(m)
                            } else {
                                Err("Bad Bishop movement")
                            }
                        },
                        Knight => {
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
                                    ! to_square.has_black()
                                {
                                self.raw_make_move(m)
                            } else {
                                Err("Bad Knight movement")
                            }
                        },
                        Pawn   => {
                            if (
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
                                self.raw_make_move(m)
                            } else {
                                Err("Bad pawn movement")
                            }
                        },
                    },
                }
            },
        }
    }
    fn raw_make_move(&mut self, m: &Move) -> Result<(), &'static str> {
        try!(self.raw_move(m));
        self.turn = !self.turn;
        Ok(())
    }
    fn raw_move(&mut self, m: &Move) -> Result<(), &'static str> {
        self.board[m.to.y][m.to.x] = self.board[m.from.y][m.from.x];
        self.board[m.from.y][m.from.x] = Some(Square::empty());
        Ok(())
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
    assert!(game.get_square(pos) == Square::white_pawn());
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

use Color::{White, Black};
#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    White,
    Black,
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
    assert!(Black == !White);
    assert!(White == !Black);
}

use PieceType::{King, Queen, Rook, Bishop, Knight, Pawn};
#[derive(Debug, PartialEq, Clone, Copy)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl PieceType {
    fn from_char(c: char) -> Result<PieceType, &'static str> {
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
    fn safe_from_char(c: char) -> PieceType {
        PieceType::from_char(c).unwrap()
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
    assert!(Rook == PieceType::safe_from_char('r'));
    assert!(Knight == PieceType::safe_from_char('n'));
    assert!(Bishop == PieceType::safe_from_char('b'));
    assert!(Queen == PieceType::safe_from_char('q'));
    assert!(King == PieceType::safe_from_char('k'));
    assert!(Pawn == PieceType::safe_from_char('p'));
    assert!(PieceType::from_char('j').is_err());
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Piece {
    tipo: PieceType,
    color: Color,
}

impl Piece {
    fn new(t: PieceType, c: Color) -> Piece {
        Piece {
            tipo: t,
            color: c,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Piece { tipo: Rook   , color: Black } => 'R',
            Piece { tipo: Knight , color: Black } => 'N',
            Piece { tipo: Bishop , color: Black } => 'B',
            Piece { tipo: Queen  , color: Black } => 'Q',
            Piece { tipo: King   , color: Black } => 'K',
            Piece { tipo: Pawn   , color: Black } => 'P',

            Piece { tipo: Rook   , color: White } => 'r',
            Piece { tipo: Knight , color: White } => 'n',
            Piece { tipo: Bishop , color: White } => 'b',
            Piece { tipo: Queen  , color: White } => 'q',
            Piece { tipo: King   , color: White } => 'k',
            Piece { tipo: Pawn   , color: White } => 'p'
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Square {
    content: Option<Piece>
}

impl Square {
    fn new(p: Option<Piece>) -> Square{
        Square { content: p }
    }

    fn has_white(&self) -> bool {
        match *self {
            Square { content: Some(Piece { tipo: _, color: White }) } => true,
            _ => false,
        }
    }
    fn has_black(&self) -> bool {
        match *self {
            Square { content: Some(Piece { tipo: _, color: Black }) } => true,
            _ => false,
        }
    }
    fn has_none(&self) -> bool {
        match *self {
            Square { content: None } => true,
            _ => false,
        }
    }
    fn has_some(&self) -> bool {
        match *self {
            Square { content: Some(_) } => true,
            _ => false,
        }
    }

    fn black_rook() -> Square {
        Square { content: Some(Piece { tipo: Rook, color: Black }) }
    }
    fn black_knight() -> Square {
        Square { content: Some(Piece { tipo: Knight, color: Black }) }
    }
    fn black_bishop() -> Square {
        Square { content: Some(Piece { tipo: Bishop, color: Black }) }
    }
    fn black_queen() -> Square {
        Square { content: Some(Piece { tipo: Queen, color: Black }) }
    }
    fn black_king() -> Square {
        Square { content: Some(Piece { tipo: King, color: Black }) }
    }
    fn black_pawn() -> Square {
        Square { content: Some(Piece { tipo: Pawn, color: Black }) }
    }

    fn white_rook() -> Square {
        Square { content: Some(Piece { tipo: Rook, color: White }) }
    }
    fn white_knight() -> Square {
        Square { content: Some(Piece { tipo: Knight, color: White }) }
    }
    fn white_bishop() -> Square {
        Square { content: Some(Piece { tipo: Bishop, color: White }) }
    }
    fn white_queen() -> Square {
        Square { content: Some(Piece { tipo: Queen, color: White }) }
    }
    fn white_king() -> Square {
        Square { content: Some(Piece { tipo: King, color: White }) }
    }
    fn white_pawn() -> Square {
        Square { content: Some(Piece { tipo: Pawn, color: White }) }
    }
    fn empty() -> Square {
        Square { content: None }
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
enum MoveType {
    Normal,
    LongCastling,
    ShortCastling,
    Promotion (PieceType),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    from: Position,
    to: Position,
    tipo: MoveType,
}

impl Move {
    fn new(from: Position, to: Position, movetype: MoveType) -> Move {
        Move {
            from: from,
            to:   to,
            tipo: movetype,
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
    fn safe_from_string(s: &str) -> Move {
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
    assert!(Move::safe_from_string("a1a1") == Move::new(Position::new(2,2), Position::new(2,2), MoveType::Normal));
    assert!(Move::from_string("z3a4").is_err());
    assert!(Move::from_string("e9a2").is_err());
    assert!(Move::from_string("aaaa").is_err());
    assert!(Move::from_string("aaa").is_err());
    assert!(Move::from_string("a").is_err());
}

type X = usize;
type Y = usize;
#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    x: X,
    y: Y,
}

impl Position {
    fn new(x: X, y: Y) -> Position {
        Position { x: x, y: y}
    }
    fn from_chars(x: char, y: char) -> Result<Position, &'static str> {
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
    fn safe_from_chars(x: char, y: char) -> Position {
        Position::from_chars(x,y).unwrap()
    }
    fn ch2y(y: char) -> Y {
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
    fn ch2x(x: char) -> X {
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
    fn up(&self) -> Position {
        Position::new(self.x, self.y + 1)
    }
    fn down(&self) -> Position {
        Position::new(self.x, self.y - 1)
    }
    fn right(&self) -> Position {
        Position::new(self.x + 1, self.y)
    }
    fn left(&self) -> Position {
        Position::new(self.x - 1, self.y)
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

#[test]
fn position_from_string() {
    assert!(Position { x: 2, y: 2 } == Position::safe_from_chars('a', '1'));
    assert!(Position::from_chars('7', '1').is_err());
}

#[test]
fn move_position() {
    assert!(Position::safe_from_chars('e','2').up().down().left().right() == Position::safe_from_chars('e','2'));
    assert!(Position::safe_from_chars('e','2').up() == Position::safe_from_chars('e','3'));
    assert!(Position::safe_from_chars('e','2').down() == Position::safe_from_chars('e','1'));
    assert!(Position::safe_from_chars('e','2').left() == Position::safe_from_chars('d','2'));
    assert!(Position::safe_from_chars('e','2').right() == Position::safe_from_chars('f','2'));
}
