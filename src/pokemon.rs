use serde::Deserialize;

/// Represents a Pokémon with its characteristics.
#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub color: String,
    pub shape: String,
    pub types: Vec<String>,
}

impl Pokemon {
    /// Displays hints about the Pokémon's characteristics.
    ///
    /// # Examples
    /// ```
    /// let pokemon = Pokemon {/*...*/}; pokemon.display_hints(); ///
    pub fn display_hints(&self) {
        println!("Hints: {}, {}, and {}", self.shape, self.color, self.types.join(", "));
    }
}
