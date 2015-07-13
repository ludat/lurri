use std::io;
mod game;
use game::{Game, Move};

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
        "" => game.show(),
        "quit" => { println!("Bye"); break },
        _ => match Move::from_string(&line) {
            Ok(ref mov) => {
                match game.make_move(&mov) {
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
