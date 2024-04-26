use crate::text_format::TextFormatter;
use rand::{thread_rng, Rng};

use crate::config::{
    ITEM_GEN_ATTACK_SPEED, ITEM_GEN_CRIT_CHANCE, ITEM_GEN_CRIT_MULTI, ITEM_GEN_POWER,
};

use super::{Item, ItemType};

pub struct Weapon {
    name: String,
    power: u16,
    attack_speed: u16,
    crit_multiplier: u16,
    crit_chance: u16,
}

impl Weapon {
    pub fn new(name: String) -> Weapon {
        let (min_power, max_power) = ITEM_GEN_POWER;
        let (min_attack_speed, max_attack_speed) = ITEM_GEN_ATTACK_SPEED;
        let (min_crit_multi, max_crit_multi) = ITEM_GEN_CRIT_MULTI;
        let (min_crit_chance, max_crit_chance) = ITEM_GEN_CRIT_CHANCE;

        let mut rng = thread_rng();

        Weapon {
            name,
            power: rng.gen_range(min_power..=max_power),
            attack_speed: rng.gen_range(min_attack_speed..=max_attack_speed),
            crit_multiplier: rng.gen_range(min_crit_multi..=max_crit_multi),
            crit_chance: rng.gen_range(min_crit_chance..=max_crit_chance),
        }
    }
}

impl Item for Weapon {
    fn name(&self) -> String {
        self.name.text_bold()
    }

    fn item_type(&self) -> ItemType {
        ItemType::Weapon
    }

    fn display_stats(&self) -> String {
        let mut stats = String::new();

        stats.push_str(&format!(
            "- Power: {}\n",
            self.power.to_string().text_bold()
        ));
        stats.push_str(&format!("- Attack Speed: {}\n", self.attack_speed));
        stats.push_str(&format!("- Crit Multiplier: {}\n", self.crit_multiplier));
        stats.push_str(&format!("- Crit Chance: {}\n", self.crit_chance));

        format!("\n{}", stats.trim())
    }
}
