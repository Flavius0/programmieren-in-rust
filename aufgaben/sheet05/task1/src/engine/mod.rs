use db::types::*;

pub mod canon;
    
#[derive(Debug, Clone)]
pub struct Pokemon {
    pub model: &'static PokemonModel,
    pub stats: Stats,
    pub level: u8,
}

impl Pokemon {
    pub fn with_level(model: &'static PokemonModel, level: u8) -> Self {
        Pokemon {
            model: model,
            level: level,
            stats: Stats::at_level(model.base_stats, level),
        }
    }

    pub fn model(&self) -> &'static PokemonModel {
        self.model
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }

    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn name(&self) -> &str {
        self.model.name
    }

    pub fn endure_attack(&mut self, attacker: &Pokemon, attack: &Attack) {
        let damage = canon::attack_damage(attacker, self, *attack);
        self.stats.hp = self.stats().hp.saturating_sub(damage);
    }

    pub fn is_alive(&self) -> bool {
        self.stats.hp > 0
    }
}
