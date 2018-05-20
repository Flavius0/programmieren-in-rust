extern crate term_painter;

mod db;
mod engine;
mod game;

use game::{execute_battle, Player};

fn main() {
    let mut p1 = Player::create_player("Red");
    let mut p2 = Player::create_player("Blue");

    println!(
        ">>>>> Status: {} has {} HP, {} has {} HP",
        p1.pokemon.name(),
        p1.pokemon.stats().hp,
        p2.pokemon.name(),
        p2.pokemon.stats().hp,
    );
    execute_battle(&mut p1, &mut p2);
}
