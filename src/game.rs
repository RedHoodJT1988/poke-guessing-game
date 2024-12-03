use reqwest::Error;
use serde::Deserialize;
use serde_json;
use rand::Rng;

use crate::pokemon::Pokemon;

#[derive(Deserialize, Debug)]
struct PokemonSpecies {
    color: Color,
    shape: Shape,
}

#[derive(Deserialize, Debug)]
struct Color {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Shape {
    name: String,
}

#[derive(Deserialize, Debug)]
struct PokemonType {
    r#type: Type,
}

#[derive(Deserialize, Debug)]
struct Type {
    name: String,
}

pub struct Game;

impl Game {
    pub async fn new() -> Result<Self, Error> {
        Ok(Game)
    }

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