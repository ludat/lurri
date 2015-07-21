use std::collections::{LinkedList};
use game::*;
// use game::PieceType::{King, Queen, Rook, Bishop, Knight, Pawn};
use game::Color::{White, Black};

extern crate rand;

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
}

#[test]
fn test_evaluate() {
    let game: Game = Game::new();
    assert_eq!(game.evaluate(), 0);
}
