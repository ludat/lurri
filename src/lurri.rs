use std::iter::Iterator;
use game::*;
use game::PieceType::{King, Queen, Rook, Bishop, Knight, Pawn};
use game::Color::{White, Black};

extern crate rand;

pub fn get_move(game: &Game) -> Move {
    let moves = game.get_valid_moves();
    // let random = rand::random::<usize>() % moves.len();
    // println!("Random = {}", random);
    println!("Posible moves:");
    for (i, mov) in moves.iter().enumerate() {
        println!("{}: {} - {} -> {}", i+1, mov, game.get_square(mov.from), game.get_square(mov.to));
    };
    moves[0]
}

impl Game {
    pub fn evaluate(&self) -> i32 {
        let mut sum = 0;
        for pos in positions() {
            match self.get_piece(pos) {
                None => {},
                Some(piece) => sum += piece.get_value(),
            }
        };
        sum
    }

    pub fn get_valid_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(75);
        for from_pos in positions() {
            if !  self.get_square(from_pos).has_color(self.turn) {
                continue
            };
            let from_square = self.get_square(from_pos);
            match from_square.get_piece() {
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
                                moves.push(Move::new(from_pos, *to_pos, MoveType::Normal));
                            }
                        }
                    }
                },
                piece!(White, Queen)  => { },
                piece!(Black, Queen)  => { },
                piece!(White, Rook)   => { },
                piece!(Black, Rook)   => { },
                piece!(White, Bishop) => { },
                piece!(Black, Bishop) => { },
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
                                moves.push(Move::new(from_pos, *to_pos, MoveType::Normal));
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
                                        moves.push(Move::new(from_pos, *to_pos,
                                            MoveType::Promotion(*promotion_piece)));
                                    }
                                } else {
                                    moves.push(Move::new(from_pos, *to_pos, MoveType::Normal))
                                }
                            }
                        }
                    }
                    let to_pos = from_pos.up();
                    if let Some(to_square) = self.get_raw_square(to_pos) {
                        if to_square.has_none() {
                            if from_pos.y==promotion_y {
                                for promotion_piece in [Queen, Rook, Bishop, Knight].iter() {
                                    moves.push(Move::new(from_pos, to_pos,
                                        MoveType::Promotion(*promotion_piece)));
                                }
                            } else {
                                moves.push(Move::new(from_pos, to_pos, MoveType::Normal))
                            }
                            let to_pos = to_pos.up(); // TODO Much nest
                            if let Some(to_square) = self.get_raw_square(to_pos) {
                                if from_pos.y == long_move_y && to_square.has_none() {
                                    moves.push(Move::new(from_pos, to_pos, MoveType::Normal));
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
                                        moves.push(Move::new(from_pos, *to_pos,
                                            MoveType::Promotion(*promotion_piece)));
                                    }
                                } else {
                                    moves.push(Move::new(from_pos, *to_pos, MoveType::Normal))
                                }
                            }
                        }
                    }
                    let to_pos = from_pos.down();
                    if let Some(to_square) = self.get_raw_square(to_pos) {
                        if to_square.has_none() {
                            if from_pos.y==promotion_y {
                                for promotion_piece in [Queen, Rook, Bishop, Knight].iter() {
                                    moves.push(Move::new(from_pos, to_pos,
                                        MoveType::Promotion(*promotion_piece)));
                                }
                            } else {
                                moves.push(Move::new(from_pos, to_pos, MoveType::Normal))
                            }
                            let to_pos = to_pos.down(); // TODO Much nest
                            if let Some(to_square) = self.get_raw_square(to_pos) {
                                if from_pos.y == long_move_y && to_square.has_none() {
                                    moves.push(Move::new(from_pos, to_pos, MoveType::Normal));
                                }
                            }
                        }
                    }
                },
            }
        };
        moves
    }
}

#[test]
fn test_evaluate() {
    let game: Game = Game::new();
    assert_eq!(game.evaluate(), 0);
}

pub struct PositionIterator {
    pub curr: Position,
}

fn positions() -> PositionIterator {
    PositionIterator { curr: Position::new(0,0)}
}

impl Iterator for PositionIterator {
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

#[test]
fn test_positions() {
    assert_eq!(positions().count(), 64);
    assert_eq!(Position::safe_from_chars('h','8'), positions().last().unwrap());
    assert_eq!(Position::safe_from_chars('a','1'), positions().nth(0).unwrap());
    assert_eq!(Position::safe_from_chars('e','1'), positions().nth(4).unwrap());
    assert_eq!(Position::safe_from_chars('a','2'), positions().nth(8).unwrap());
    assert_eq!(Position::safe_from_chars('a','5'), positions().nth(32).unwrap());
    assert_eq!(Position::safe_from_chars('h','8'), positions().nth(63).unwrap());
}
