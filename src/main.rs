#![allow(dead_code)]
use std::fmt;
use std::io;
use std::ops::Not;

type Board = [[Option<Square>; 12]; 12];

struct Game {
    board: Board,
    turn: Color,
}

impl Game {
    fn new() -> Game {
        Game {
            board: [
                [None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None],
                [ // 1
                    None, None,
                    Some(Square::white_root()),
                    Some(Square::white_knight()),
                    Some(Square::white_bishop()),
                    Some(Square::white_queen()),
                    Some(Square::white_king()),
                    Some(Square::white_bishop()),
                    Some(Square::white_knight()),
                    Some(Square::white_root()),
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
                    Some(Square::black_root()),
                    Some(Square::black_knight()),
                    Some(Square::black_bishop()),
                    Some(Square::black_queen()),
                    Some(Square::black_king()),
                    Some(Square::black_bishop()),
                    Some(Square::black_knight()),
                    Some(Square::black_root()),
                    None, None,
                ],

                [None, None, None, None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None, None, None, None],
            ],
            turn: White,
        }
    }
    fn show(&self) {
        println!("{}", self)
    }
    fn makemove(&mut self, m: &Move) -> Result<(), &'static str> {
        println!("Making move {}...", m);
        match self.board[m.from.y][m.from.x] {
            None => Err("Not even a valid square"),
            Some(Square { piece: None } ) => Err("Empty from square"),
            Some(Square { piece: Some(Piece { tipo: pt, color: c }) }) => {
                if c != self.turn {
                    return Err("Wrong color")
                };
                self.board[m.to.y][m.to.x] = self.board[m.from.y][m.from.x];
                self.board[m.from.y][m.from.x] = Some(Square::new(None));
                self.turn = !self.turn;
                Ok(())
            },
        }
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
    assert!(Rook == PieceType::from_char('r').unwrap());
    assert!(Knight == PieceType::from_char('n').unwrap());
    assert!(Bishop == PieceType::from_char('b').unwrap());
    assert!(Queen == PieceType::from_char('q').unwrap());
    assert!(King == PieceType::from_char('k').unwrap());
    assert!(Pawn == PieceType::from_char('p').unwrap());
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
    piece: Option<Piece>
}

impl Square {
    fn new(p: Option<Piece>) -> Square{
        Square { piece: p }
    }

    fn black_root () -> Square {
        Square { piece: Some(Piece { tipo: Rook, color: Black }) }
    }
    fn black_knight () -> Square {
        Square { piece: Some(Piece { tipo: Knight, color: Black }) }
    }
    fn black_bishop () -> Square {
        Square { piece: Some(Piece { tipo: Bishop, color: Black }) }
    }
    fn black_queen () -> Square {
        Square { piece: Some(Piece { tipo: Queen, color: Black }) }
    }
    fn black_king () -> Square {
        Square { piece: Some(Piece { tipo: King, color: Black }) }
    }
    fn black_pawn () -> Square {
        Square { piece: Some(Piece { tipo: Pawn, color: Black }) }
    }

    fn white_root () -> Square {
        Square { piece: Some(Piece { tipo: Rook, color: White }) }
    }
    fn white_knight () -> Square {
        Square { piece: Some(Piece { tipo: Knight, color: White }) }
    }
    fn white_bishop () -> Square {
        Square { piece: Some(Piece { tipo: Bishop, color: White }) }
    }
    fn white_queen () -> Square {
        Square { piece: Some(Piece { tipo: Queen, color: White }) }
    }
    fn white_king () -> Square {
        Square { piece: Some(Piece { tipo: King, color: White }) }
    }
    fn white_pawn () -> Square {
        Square { piece: Some(Piece { tipo: Pawn, color: White }) }
    }
    fn empty () -> Square {
        Square { piece: None }
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Square { piece: Some(ref p) } => format!("{}", p),
            Square { piece: None } => format!("."),
        })
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum MoveType {
    Normal,
    LongCastling,
    ShortCastling,
    Promotion (PieceType),
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Move {
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
    fn from_string(s: &str) -> Result<Move, &'static str> {
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
    assert!(Move::from_string("a1a1").unwrap() == Move::new(Position::new(2,2), Position::new(2,2), MoveType::Normal));
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
                'a' => 2,
                'b' => 3,
                'c' => 4,
                'd' => 5,
                'e' => 6,
                'f' => 7,
                'g' => 8,
                'h' => 9,
                 _  => return Err("Bad letter"),
            },
            y: match y {
                '1' => 2,
                '2' => 3,
                '3' => 4,
                '4' => 5,
                '5' => 6,
                '6' => 7,
                '7' => 8,
                '8' => 9,
                 _  => return Err("Bad Number"),
            },
        })
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
    assert!(Position { x: 2, y: 2 } == Position::from_chars('a', '1').unwrap());
    assert!(Position::from_chars('7', '1').is_err());
}

#[test]
fn move_position() {
    assert!(Position::from_chars('e','2').unwrap().up().down().left().right() == Position::from_chars('e','2').unwrap());
    assert!(Position::from_chars('e','2').unwrap().up() == Position::from_chars('e','3').unwrap());
    assert!(Position::from_chars('e','2').unwrap().down() == Position::from_chars('e','1').unwrap());
    assert!(Position::from_chars('e','2').unwrap().left() == Position::from_chars('d','2').unwrap());
    assert!(Position::from_chars('e','2').unwrap().right() == Position::from_chars('f','2').unwrap());
}

fn main() {
    println!("This is my adorable chess engine written in awesome Rust");
    println!("Write your moves like e2e4. You are white by default");
    let mut game: Game = Game::new();
    game.show();
    loop {
        let mut line: String = String::new();
        if let Err(..) = io::stdin().read_line(&mut line) {
            println!("Nope string");
            continue;
        };
        line.pop(); // Remove the new line character
        match line.as_ref() {
        "new"  => game = Game::new(),
        "show" => game.show(),
        "quit" => { println!("Bye"); break },
        _ => match Move::from_string(&line) {
            Ok(ref mov) => {
                match game.makemove(&mov) {
                    Ok(()) => { println!("Move made"); game.show() },
                    Err(e) => println!("Couldn't make move, {}", e),
                }
            },
            Err(ref e) => {
                    println!("Couldn't read move, {}", e)
                },
            },
        }
    }
}
