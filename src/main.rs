#![feature(test)]
use std::io;
#[macro_use]
mod game;
mod lurri;
use game::{Game, Move};
use game::Color::{White, Black};

fn main() {
    println!("This is my adorable chess engine written in awesome Rust");
    println!("Write your moves like e2e4. You are white by default");
    let mut game: Game = Game::new();
    let mut engine_color = Black;
    loop {
        game.show();
        if game.turn == engine_color {
            let engine_move: Move = lurri::get_move(&game);
            match game.make_move(&engine_move) {
                Err(ref e) => {
                    println!("Lurri couldn't make move, {}", e)
                },
                Ok(_) => {
                    println!("Lurri has moved")
                },
            }
        };

        let mut line: String = String::new();
        if let Err(..) = io::stdin().read_line(&mut line) {
            println!("Nope string");
            continue;
        };
        line.pop(); // Remove the new line character

        match line.as_ref() {
            "new"  => game = Game::new(),
            "quit" => { println!("Bye"); break },
            "white" => engine_color = White,
            "black" => engine_color = Black,
            "xboard" => return xboard(),
            "l" => {
                let engine_move: Move = lurri::get_move(&game);
                match game.make_move(&engine_move) {
                    Err(ref e) => {
                        println!("Lurri couldn't make move, {}", e)
                    },
                    Ok(_) => {
                        println!("Lurri has moved")
                    },
                }
            },
            _ => match Move::from_string(&line) {
                Ok(ref mov) => {
                    match game.make_move(&mov) {
                        Ok(_) => { println!("Move made"); game.show() },
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

fn xboard() {
    let mut game: Game = Game::new();
    let mut engine_color = Black;
    loop {
        if game.turn == engine_color {
            println!("# lurri should think");
            let engine_move: Move = lurri::get_move(&game);
            match game.make_move(&engine_move) {
                Ok(_) => {
                    println!("move {}", engine_move);
                },
                Err(e) => {
                    println!("# telluser error: {}", e)
                },
            }
        };
        let mut line: String = String::new();
        if let Err(..) = io::stdin().read_line(&mut line) {
            println!("# failed to read input");
            continue;
        }; line.pop();

        match line.as_ref() {
            "new"  => game = Game::new(),
            "white" => engine_color = White,
            "black" => engine_color = Black,
            "quit" => break,
            "edit" => edit_mode(&mut game),
            _ => match Move::from_string(&line) {
                Ok(mov) => {
                    match game.make_move(&mov) {
                        Ok(_) => println!("# telluser You made a move"),
                        Err(e) => println!("# Illegal move ({}): {}", e, mov),
                    }
                },
                Err(ref e) => {
                    println!("# Error ({}): {}", e, line)
                },
            },
        }
    }
}

fn edit_mode(game: &mut Game) {
    let mut curr_color = White;
    loop {
        let mut line: String = String::new();
        if let Err(..) = io::stdin().read_line(&mut line) {
            continue;
        }; line.pop();

        match line.as_ref() {
            "c"  => curr_color = ! curr_color,
            "." => break,
            _ => match Move::from_string(&line) {
                Ok(ref mov) => {
                    match game.make_move(&mov) {
                        Ok(()) => println!("telluser You made a move"),
                        Err(e) => println!("Illegal move ({}): {}", e, mov),
                    }
                },
                Err(ref e) => {
                    println!("Error ({}): {}", e, line)
                },
            },
        }
    }
}
