use std::collections::{LinkedList};
use game::*;
// use game::PieceType::{King, Queen, Rook, Bishop, Knight, Pawn};
use game::Color::{White, Black};

extern crate rand;
extern crate test;

pub fn get_move(game: &Game) -> Move {
    get_best_move(game)
}

pub fn get_best_move(game: &Game) -> Move {
    let mut moves = game.get_all_valid_moves();
    game.evaluate_moves(&mut moves);
    let mov: ValuedMove = match game.turn {
        White => *moves.iter().max().unwrap(),
        Black => *moves.iter().min().unwrap(),
    };
    mov.mov

}

impl Game {
    pub fn evaluate(&self) -> i32 {
        Position::all().fold(0, |acc, pos|
            match self.get_piece(pos) {
                None => acc,
                Some(piece) =>
                    acc + piece.get_value() * 10
                    + piece.color.get_sign() * (self.get_valid_moves(pos).len() as i32)
                    ,
            }
        )
    }
    pub fn evaluate_move(&self, mov: &Move) -> i32 {
        let mut aux_game = (*self).clone();
        if let Err(e) = aux_game.make_move(mov) {
            println!("ERROR (engine generated an invalid move) mov: {}, reason: {}", mov, e)
        }
        aux_game.evaluate()
    }
    pub fn evaluate_moves<'a>(&self, moves: &'a mut LinkedList<ValuedMove>) -> &'a mut LinkedList<ValuedMove> {
        for mov in moves.iter_mut() {
            mov.value = Some(self.evaluate_move(&mov.mov))
        }
        moves
    }
}

#[test]
fn test_evaluate() {
    let game: Game = Game::new();
    assert_eq!(game.evaluate(), 0);
}

#[bench]
fn bench_evaluate(b: &mut test::Bencher) {
    let game: Game = Game::new();
    b.iter(|| game.evaluate());
}
#[bench]
fn bench_get_all_valid_moves(b: &mut test::Bencher) {
    let game: Game = Game::new();
    b.iter(|| game.get_all_valid_moves());
}
#[bench]
fn bench_evaluate_linkedlist(b: &mut test::Bencher) {
    let game: Game = Game::new();
    b.iter(|| {
        let mut moves = game.get_all_valid_moves();
        game.evaluate_moves(&mut moves);
    });
}
#[bench]
fn bench_get_move(b: &mut test::Bencher) {
    let game: Game = Game::new();
    b.iter(|| get_move(&game));
}
