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
        for pos in Position::all() {
            match self.get_piece(pos) {
                None => {},
                Some(piece) => sum += piece.get_value(),
            }
        };
        sum
    }

    pub fn get_valid_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(75);
        for from_pos in Position::all() {
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
                piece!(color, Queen)  => {
                    for dir in [
                            Direction::Up, Direction::Down,
                            Direction::Left, Direction::Right,
                            Direction::UpRight, Direction::UpLeft,
                            Direction::DownRight, Direction::DownLeft].iter(){
                        for to_pos in from_pos.iter_to(*dir){
                            match self.get_raw_square(to_pos) {
                                Some(to_square) if ! to_square.has_color(color) => {
                                    moves.push(Move::new(from_pos, to_pos, MoveType::Normal));
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
                                Some(to_square) if ! to_square.has_color(color) => {
                                    moves.push(Move::new(from_pos, to_pos, MoveType::Normal));
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
                                Some(to_square) if ! to_square.has_color(color) => {
                                    moves.push(Move::new(from_pos, to_pos, MoveType::Normal));
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
