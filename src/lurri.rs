use game::*;
// use game::PieceType::{King, Queen, Rook, Bishop, Knight, Pawn};
use game::Color::{White, Black};
use game::BoardValue::{WonWhite, Value, WonBlack, Invalid};

extern crate test;

pub fn get_move(game: &Game, max_ply: u32) -> Move {
    get_best_move(game, 0, max_ply).mov
}

pub fn get_best_move(game: &Game, ply: u32, max_ply: u32) -> ValuedMove {
    let mut moves: Vec<ValuedMove> = game.get_all_valid_moves();
    for mov in moves.iter_mut() {
        let mut aux_game: Game = game.clone();
        if ply == max_ply {
            mov.value = aux_game.evaluate_move(&mov.mov)
        } else {
            if let Err(..) = aux_game.make_move(&mov.mov) {
                mov.value = Invalid;
                continue
            };
            mov.value = get_best_move(&aux_game, ply + 1, max_ply).value
        }
    }

    if ! moves.iter().any(|mov| mov.value.is_valid()) {
        return ValuedMove::from_value(
            match game.turn {
                White => WonBlack,
                Black => WonWhite,
            }
        )
    };
    let mov: ValuedMove = match game.turn {
        White => *moves.iter().filter(|mov| mov.value.is_valid()).max().unwrap(),
        Black => *moves.iter().filter(|mov| mov.value.is_valid()).min().unwrap(),
    };
    mov
}

impl Game {
    pub fn evaluate(&self) -> i32 {
        let mut moves = Vec::with_capacity(80);
        Position::all().fold(0, |acc, pos|
            match self.get_piece(pos) {
                None => acc,
                Some(piece) => {
                    moves.clear();
                    acc
                    + piece.get_value() * 10
                    + piece.color.get_sign() * (self.get_valid_moves(pos, &mut moves).len() as i32)
                },
            }
        )
    }
    pub fn evaluate_move(&self, mov: &Move) -> BoardValue {
        let mut aux_game = (*self).clone();
        if let Err(..) = aux_game.make_move(mov) {
            // println!("ERROR (engine generated an invalid move) mov: {}, reason: {}", mov, e);
            Invalid
        } else {
            Value(aux_game.evaluate())
        }
    }
    pub fn evaluate_moves<'a>(&self, moves: &'a mut Vec<ValuedMove>) -> &'a mut Vec<ValuedMove> {
        for mov in moves.iter_mut() {
            mov.value = self.evaluate_move(&mov.mov)
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
fn bench_evaluate_list(b: &mut test::Bencher) {
    let game: Game = Game::new();
    let moves = game.get_all_valid_moves();
    b.iter(|| {
        game.evaluate_moves(&mut moves.clone());
    });
}
#[bench]
fn bench_get_move(b: &mut test::Bencher) {
    let game: Game = Game::new();
    b.iter(|| get_move(&game, 3));
}
// #[bench]
// fn bench_game(b: &mut test::Bencher) {
//     b.iter(|| {
//         let mut game: Game = Game::new();
//         let engine_move = get_move(&game, 3);
//         game.make_move(&engine_move).unwrap();
//         let engine_move = get_move(&game, 3);
//         game.make_move(&engine_move).unwrap();
//         let engine_move = get_move(&game, 3);
//         game.make_move(&engine_move).unwrap();
//         let engine_move = get_move(&game, 3);
//         game.make_move(&engine_move).unwrap();
//         let engine_move = get_move(&game, 3);
//         game.make_move(&engine_move).unwrap();
//         let engine_move = get_move(&game, 3);
//         game.make_move(&engine_move).unwrap();
//         let engine_move = get_move(&game, 3);
//         game.make_move(&engine_move).unwrap();
//         let engine_move = get_move(&game, 3);
//         game.make_move(&engine_move).unwrap();
//         let engine_move = get_move(&game, 3);
//         game.make_move(&engine_move).unwrap();
//     });
// }
