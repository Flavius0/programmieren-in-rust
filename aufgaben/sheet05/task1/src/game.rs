use std;

use term_painter::ToStyle;
use term_painter::Color::*;
use term_painter::Attr::*;

use db::data::POKEDEX;
use db::types::*;
use db::find_pokemon_by_name;

use engine::Pokemon;


#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub pokemon: Pokemon,
}

impl Player {
    pub fn create_player(name: &str) -> Self {
        Player {
            name: name.into(),
            pokemon: Self::choose_pokemon(name),
        }
    }

    pub fn choose_pokemon(name: &str) -> Pokemon {
        loop {
            println!(
                "Player {}, please choose a Pokemon (or type '?' to get a complete list)",
                name
            );
            let poke_name = read_string();
            if poke_name == "?" {
                print_pokemon_list();
                continue;
            }

            let o_model = find_pokemon_by_name(poke_name);
            match o_model {
                None => println!("Please choose an existing pokemon"),
                Some(model) => return Pokemon::with_level(model, 5),
            }
        }
    }
}


pub fn execute_battle(p1: &mut Player, p2: &mut Player) {
    let attacker;
    let defender;

    if p1.pokemon.stats().speed >= p2.pokemon.stats().speed {
        attacker = p1;
        defender = p2;
    } else {
        attacker = p2;
        defender = p1;
    }

    loop {
        println!(
            ">>>>> {} is about to attack! Which move shall it execute?",
            Bold.paint(attacker.pokemon.name())
        );
        {
            let attack = get_attack_by_uid(&attacker.pokemon);
            defender.pokemon.endure_attack(&attacker.pokemon, attack);
            println!(
                ">>>>> {} uses {}! ({} has {} HP left)",
                Bold.paint(attacker.pokemon.name()),
                Red.paint(attack.name),
                Bold.paint(defender.pokemon.name()),
                defender.pokemon.stats().hp,
            );
        }
        if !defender.pokemon.is_alive() {
            println!(">>>>> {} fainted!", defender.pokemon.name());
            break;
        }
        std::mem::swap(attacker, defender);
    }
}

fn print_pokemon_list() {
    for &p in POKEDEX {
        println!("#{:03} {}", p.id, p.name);
    }
}

/// Reads a string from the terminal/user.
fn read_string() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("something went horribly wrong...");

    // Discard trailing newline
    let new_len = buffer.trim_right().len();
    buffer.truncate(new_len);

    buffer
}

/// Reads a valid `usize` integer from the terminal/user.
fn read_usize() -> usize {
    loop {
        match read_string().parse() {
            Ok(res) => return res,
            Err(_) => println!("That was not an integer! Please try again!"),
        }
    }
}

pub fn get_attack_by_uid(pokemon: &Pokemon) -> &Attack {
    let attacks = pokemon.model().attacks;
    for (i, attack) in attacks.iter().enumerate() {
        println!("    {}: {}", i, attack.name);
    }
    loop {
        println!("    !!! Please give me the attack ID:");
        let attack_id = read_usize();
        if attack_id < attacks.len() {
            return attacks[attack_id];
        }
    }
}
