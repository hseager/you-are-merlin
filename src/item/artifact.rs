use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

use crate::config::{
    ITEM_GEN_ATTACK_SPEED, ITEM_GEN_DODGE_CHANCE, ITEM_GEN_MAX_LIFE, ITEM_GEN_POWER,
};

use super::{
    get_rarity_property_count, get_rarity_text_color, get_reward_item_rarity, Item, ItemRarity,
    ItemStat, ItemType,
};

// TODO Change these to more interesting stats
pub struct Artifact {
    name: String,
    rarity: ItemRarity,
    power: u16,
    attack_speed: u16,
    max_life: u16,
    dodge_chance: u16,
}

impl Artifact {
    pub fn new(name: String) -> Artifact {
        let mut rng = thread_rng();

        let mut properties = [
            (
                ItemStat::Power,
                rng.gen_range(ITEM_GEN_POWER.0..=ITEM_GEN_POWER.1),
            ),
            (
                ItemStat::AttackSpeed,
                rng.gen_range(ITEM_GEN_ATTACK_SPEED.0..=ITEM_GEN_ATTACK_SPEED.1),
            ),
            (
                ItemStat::MaxLife,
                rng.gen_range(ITEM_GEN_MAX_LIFE.0..=ITEM_GEN_MAX_LIFE.1),
            ),
            (
                ItemStat::DodgeChance,
                rng.gen_range(ITEM_GEN_DODGE_CHANCE.0..=ITEM_GEN_DODGE_CHANCE.1),
            ),
        ];

        properties.shuffle(&mut rng);

        let rarity = get_reward_item_rarity();

        let selected_properties = properties.iter().take(get_rarity_property_count(&rarity));

        let mut artifact = Artifact {
            name,
            rarity,
            power: 0,
            attack_speed: 0,
            max_life: 0,
            dodge_chance: 0,
        };

        for (property, value) in selected_properties {
            match property {
                ItemStat::Power => artifact.power = *value,
                ItemStat::AttackSpeed => artifact.attack_speed = *value,
                ItemStat::MaxLife => artifact.max_life = *value,
                ItemStat::DodgeChance => artifact.dodge_chance = *value,
                _ => unreachable!(),
            }
        }

        artifact
    }
}

impl Item for Artifact {
    fn name(&self) -> String {
        get_rarity_text_color(&self.rarity, &self.name)
    }

    fn item_type(&self) -> ItemType {
        ItemType::Artifact
    }

    fn display_info(&self) -> String {
        let mut stats = String::new();

        stats.push_str(&format!(
            "{} - ({})\n",
            self.name(),
            &self.rarity.to_string()
        ));

        if self.power > 0 {
            stats.push_str(&format!("- Power: {}\n", self.power));
        }
        if self.attack_speed > 0 {
            stats.push_str(&format!("- Attack Speed: {}\n", self.attack_speed));
        }
        if self.max_life > 0 {
            stats.push_str(&format!("- Max Life: {}\n", self.max_life));
        }
        if self.dodge_chance > 0 {
            stats.push_str(&format!("- Dodge Chance: {}\n", self.dodge_chance));
        }

        format!("\n{}", stats.trim())
    }
}
