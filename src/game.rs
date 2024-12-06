use reqwest::Error;
use serde::Deserialize;
use serde_json;
use rand::Rng;

use crate::pokemon::Pokemon;

/// Represents a Pokémon's species information.
#[derive(Deserialize, Debug)]
struct PokemonSpecies {
    color: Color,
    shape: Shape,
}

/// Represents the color of a Pokémon
#[derive(Deserialize, Debug)]
struct Color {
    name: String,
}

/// Represents the shape of a Pokémon
#[derive(Deserialize, Debug)]
struct Shape {
    name: String,
}

/// Represents a Pokémon type
#[derive(Deserialize, Debug)]
struct PokemonType {
    r#type: Type,
}

/// Represents a single type (e.g., Fire, Water).
#[derive(Deserialize, Debug)]
struct Type {
    name: String,
}

/// Represents a Pokémon generation.
#[derive(Deserialize, Debug)]
struct Generation {
    name: String,
}

pub struct Game;

impl Game {
    /// Creates a new instance of `Game`.
    pub async fn new() -> Result<Self, Error> {
        Ok(Game)
    }

    /// Fetches a random Pokémon from the API and returns its information.
    ///
    /// # Returns
    /// * `Ok(Pokemon)` if successful
    /// * `Err(Error)` if there was an error fetching the data
    pub async fn fetch_random_pokemon(&self) -> Result<Pokemon, Error> {
        let pokemon_id = rand::thread_rng().gen_range(1..=898); // There are 898 Pokémon as of now
        let pokemon_url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id);
        let species_url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", pokemon_id);

        let pokemon_response = reqwest::get(&pokemon_url).await?.json::<serde_json::Value>().await?;
        let species_response = reqwest::get(&species_url).await?.json::<PokemonSpecies>().await?;

        let types: Vec<String> = pokemon_response["types"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["type"]["name"].as_str().unwrap().to_string())
            .collect();

        Ok(Pokemon {
            name: pokemon_response["name"].as_str().unwrap().to_string(),
            color: species_response.color.name,
            shape: species_response.shape.name,
            types,
        })
    }
}
