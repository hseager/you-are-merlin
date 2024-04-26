use crate::text_format::TextFormatter;
use rand::{thread_rng, Rng};

use crate::config::{
    ITEM_GEN_ATTACK_SPEED, ITEM_GEN_DODGE_CHANCE, ITEM_GEN_MAX_LIFE, ITEM_GEN_POWER,
};

use super::{Item, ItemType};

// TODO Change these to more interesting stats
pub struct Artifact {
    name: String,
    power: u16,
    attack_speed: u16,
    max_life: u16,
    dodge_chance: u16,
}

impl Artifact {
    pub fn new(name: String) -> Artifact {
        let (min_power, max_power) = ITEM_GEN_POWER;
        let (min_attack_speed, max_attack_speed) = ITEM_GEN_ATTACK_SPEED;
        let (min_life, max_life) = ITEM_GEN_MAX_LIFE;
        let (min_dodge_chance, max_dodge_chance) = ITEM_GEN_DODGE_CHANCE;

        let mut rng = thread_rng();

        Artifact {
            name,
            power: rng.gen_range(min_power..=max_power),
            attack_speed: rng.gen_range(min_attack_speed..=max_attack_speed),
            max_life: rng.gen_range(min_life..=max_life),
            dodge_chance: rng.gen_range(min_dodge_chance..=max_dodge_chance),
        }
    }
}

impl Item for Artifact {
    fn name(&self) -> String {
        self.name.text_bold()
    }

    fn item_type(&self) -> ItemType {
        ItemType::Artifact
    }

    fn display_info(&self) -> String {
        let mut stats = String::new();

        stats.push_str(&format!("- Power: {}\n", self.power));
        stats.push_str(&format!("- Attack Speed: {}\n", self.attack_speed));
        stats.push_str(&format!("- Max Life: {}\n", self.max_life));
        stats.push_str(&format!("- Dodge Chance: {}\n", self.dodge_chance));

        format!("\n{}", stats.trim())
    }
}
