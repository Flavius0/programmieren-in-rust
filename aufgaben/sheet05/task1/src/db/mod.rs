pub mod types;
pub mod data;

pub use self::types::*;

pub fn find_pokemon_by_name(name: String) -> Option<&'static PokemonModel> {
    let t = name.to_lowercase();
    for p in data::POKEDEX {
        if p.name.to_lowercase() == t {
            return Some(p);
        }
    }
    return None;
}
