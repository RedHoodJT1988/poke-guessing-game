mod game;
mod pokemon;

use std::io::{self, Write};
use std::sync::Once;

use game::Game;

static INIT: Once = Once::new();

fn init() {
    io::stdout().flush().unwrap();
}

#[tokio::main]
async fn main() {
    INIT.call_once(|| {
        init();
    });

    let game = match Game::new().await {
        Ok(game) => game,
        Err(e) => {
            eprintln!("Failed to initialize game: {}", e);
            return;
        }
    };

    let mut score = 0;
    let rounds = 5;

    for _ in 0..rounds {
        let pokemon = match game.fetch_random_pokemon().await {
            Ok(pokemon) => pokemon,
            Err(e) => {
                eprintln!("Failed to fetch Pokémon: {}", e);
                continue;
            }
        };

        pokemon.display_hints();

        println!("Guess the Pokémon:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        if guess.trim().eq_ignore_ascii_case(&pokemon.name) {
            println!("Correct!");
            score += 1;
        } else {
            println!("Incorrect! The correct answer was: {}", pokemon.name);
        }
    }

    println!("You won {} points. Good job!", score);
}