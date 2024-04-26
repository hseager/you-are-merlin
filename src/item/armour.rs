use crate::text_format::TextFormatter;
use rand::{thread_rng, Rng};

use crate::config::{ITEM_GEN_BLOCK, ITEM_GEN_DODGE_CHANCE, ITEM_GEN_MAX_LIFE};

use super::{Item, ItemType};

pub struct Armour {
    name: String,
    max_life: u16,
    block: u16,
    dodge_chance: u16,
}

impl Armour {
    pub fn new(name: String) -> Armour {
        let (min_life, max_life) = ITEM_GEN_MAX_LIFE;
        let (min_block, max_block) = ITEM_GEN_BLOCK;
        let (min_dodge_chance, max_dodge_chance) = ITEM_GEN_DODGE_CHANCE;

        let mut rng = thread_rng();

        Armour {
            name,
            max_life: rng.gen_range(min_life..=max_life),
            block: rng.gen_range(min_block..=max_block),
            dodge_chance: rng.gen_range(min_dodge_chance..=max_dodge_chance),
        }
    }
}

impl Item for Armour {
    fn name(&self) -> String {
        self.name.text_bold()
    }

    fn item_type(&self) -> ItemType {
        ItemType::Armour
    }

    fn display_stats(&self) -> String {
        let mut stats = String::new();

        stats.push_str(&format!("- Max Life: {}\n", self.max_life));
        stats.push_str(&format!("- Block Chance: {}\n", self.block));
        stats.push_str(&format!("- Dodge Chance: {}\n", self.dodge_chance));

        format!("\n{}", stats.trim())
    }
}
