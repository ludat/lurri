use std::collections::{LinkedList};
use std::cmp;
use std::cmp::{Ord, Eq, PartialOrd, Ordering};
use game::*;
use game::PieceType::{King, Queen, Rook, Bishop, Knight, Pawn};
use game::Color::{White, Black};

extern crate rand;

#[derive(Debug, Clone, Copy)]
pub struct ValuedMove {
    value: Option<i32>,
    mov: Move,
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
fn test_partialOrd() {
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

pub fn get_move(game: &Game) -> Move {
    get_best_move(game, 1, 1)
}

pub fn get_best_move(game: &Game, final_depth: u32, current_depth: u32) -> Move {
    let mut moves = game.get_all_valid_moves();
    game.evaluate_moves(&mut moves);
    let mov: ValuedMove = match game.turn {
        White => *moves.iter().max().unwrap(),
        Black => *moves.iter().min().unwrap(),
    };
    println!("My move will be {}", mov.mov);
    mov.mov

}

impl Game {
    pub fn evaluate(&self) -> i32 {
        let mut sum = 0;
        for pos in Position::all() {
            match self.get_piece(pos) {
                None => {},
                Some(piece) => {
                    sum += piece.get_value() * 10 + piece.color.get_sign() * (self.get_valid_moves(pos).len() as i32)
                },

            }
        };
        sum
    }
    pub fn evaluate_move(&self, mov: &Move) -> i32 {
        let mut aux_game = (*self).clone();
        if let Err(e) = aux_game.make_move(mov) {
            println!("ERROR mov: {}, reason: {}", mov, e)
        }
        aux_game.evaluate()
    }
    pub fn evaluate_moves<'a>(&'a self, moves: &'a mut LinkedList<ValuedMove>) -> &'a mut LinkedList<ValuedMove> {
        for mov in moves.iter_mut() {
            if mov.value.is_none() {
                mov.value = Some(self.evaluate_move(&mov.mov))
            } else {
                panic!("tried to value an already valued move")
            }
        }
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
}

#[test]
fn test_evaluate() {
    let game: Game = Game::new();
    assert_eq!(game.evaluate(), 0);
}
