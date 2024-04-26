use crate::{config::ITEM_GEN_PARRY_CHANCE, text_format::TextFormatter};
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

use crate::config::{ITEM_GEN_BLOCK, ITEM_GEN_DODGE_CHANCE, ITEM_GEN_MAX_LIFE};

use super::{get_rarity_property_count, get_reward_item_rarity, Item, ItemRarity, ItemType};

pub struct Armour {
    name: String,
    rarity: ItemRarity,
    max_life: u16,
    block: u16,
    parry_chance: u16,
    dodge_chance: u16,
}

impl Armour {
    pub fn new(name: String) -> Armour {
        let (min_life, max_life) = ITEM_GEN_MAX_LIFE;
        let (min_block, max_block) = ITEM_GEN_BLOCK;
        let (min_parry_chance, max_parry_chance) = ITEM_GEN_PARRY_CHANCE;
        let (min_dodge_chance, max_dodge_chance) = ITEM_GEN_DODGE_CHANCE;

        let mut rng = thread_rng();

        let mut properties = [
            ("max_life", rng.gen_range(min_life..=max_life)),
            ("block", rng.gen_range(min_block..=max_block)),
            (
                "parry_chance",
                rng.gen_range(min_parry_chance..=max_parry_chance),
            ),
            (
                "dodge_chance",
                rng.gen_range(min_dodge_chance..=max_dodge_chance),
            ),
        ];

        properties.shuffle(&mut rng);

        let rarity = get_reward_item_rarity();

        let selected_properties = properties.iter().take(get_rarity_property_count(&rarity));

        let mut armour = Armour {
            name,
            rarity,
            max_life: 0,
            block: 0,
            parry_chance: 0,
            dodge_chance: 0,
        };

        for &(property, value) in selected_properties {
            match property {
                "max_life" => armour.max_life = value,
                "block" => armour.block = value,
                "parry_chance" => armour.parry_chance = value,
                "dodge_chance" => armour.dodge_chance = value,
                _ => unreachable!(), // Error if unexpected property is encountered
            }
        }

        armour
    }
}

impl Item for Armour {
    fn name(&self) -> String {
        self.name.text_bold()
    }

    fn item_type(&self) -> ItemType {
        ItemType::Armour
    }

    fn display_info(&self) -> String {
        let mut stats = String::new();

        stats.push_str(&format!(
            "{} - ({})\n",
            self.name(),
            &self.rarity.to_string()
        ));

        if self.max_life > 0 {
            stats.push_str(&format!("- Max Life: {}\n", self.max_life));
        }

        if self.block > 0 {
            stats.push_str(&format!("- Block: {}\n", self.block));
        }

        if self.parry_chance > 0 {
            stats.push_str(&format!("- Parry: {}\n", self.parry_chance));
        }

        if self.dodge_chance > 0 {
            stats.push_str(&format!("- Dodge Chance: {}\n", self.dodge_chance));
        }

        format!("\n{}", stats.trim())
    }
}
