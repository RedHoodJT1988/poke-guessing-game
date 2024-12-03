use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub color: String,
    pub shape: String,
    pub types: Vec<String>,
}

impl Pokemon {
    pub fn display_hints(&self) {
        println!("Hints: {}, {}, and {}", self.shape, self.color, self.types.join(", "));
    }
}