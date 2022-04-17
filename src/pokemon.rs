use rust_embed::EmbeddedFile;
use serde::Deserialize;

use std::collections::HashMap;
use std::error::Error;
use std::str;

#[derive(Debug, Deserialize)]
pub struct Pokemon {
    pub idx: u32,
    pub slug: String,
    pub gen: u8,
    pub form: String,
    pub name: HashMap<String, String>,
    pub desc: HashMap<String, String>,
}

pub fn load_pokemon(pokemon_db: &EmbeddedFile) -> Result<Vec<Pokemon>, Box<dyn Error>> {
    let pokemon_json_str = str::from_utf8(&pokemon_db.data)?;
    let pokemon: Vec<Pokemon> = serde_json::from_str(&pokemon_json_str)?;
    Ok(pokemon)
}
