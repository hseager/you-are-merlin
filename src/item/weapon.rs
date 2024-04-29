use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

use crate::config::{
    ITEM_GEN_ATTACK_SPEED, ITEM_GEN_CRIT_CHANCE, ITEM_GEN_CRIT_MULTI, ITEM_GEN_POWER,
};

use super::{
    get_rarity_property_count, get_rarity_text_color, get_reward_item_rarity, Item, ItemRarity,
    ItemStat, ItemType,
};

pub struct Weapon {
    name: String,
    rarity: ItemRarity,
    power: u16,
    attack_speed: u16,
    crit_multiplier: u16,
    crit_chance: u16,
}

impl Weapon {
    pub fn new(name: String) -> Weapon {
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
                ItemStat::CritMultiplier,
                rng.gen_range(ITEM_GEN_CRIT_MULTI.0..=ITEM_GEN_CRIT_MULTI.1),
            ),
            (
                ItemStat::CritChance,
                rng.gen_range(ITEM_GEN_CRIT_CHANCE.0..=ITEM_GEN_CRIT_CHANCE.1),
            ),
        ];

        properties.shuffle(&mut rng);

        let rarity = get_reward_item_rarity();

        let selected_properties = properties.iter().take(get_rarity_property_count(&rarity));

        let mut weapon = Weapon {
            name,
            rarity,
            power: 0,
            attack_speed: 0,
            crit_multiplier: 0,
            crit_chance: 0,
        };

        for (property, value) in selected_properties {
            match property {
                ItemStat::Power => weapon.power = *value,
                ItemStat::AttackSpeed => weapon.attack_speed = *value,
                ItemStat::CritMultiplier => weapon.crit_multiplier = *value,
                ItemStat::CritChance => weapon.crit_chance = *value,
                _ => unreachable!(),
            }
        }

        weapon
    }
}

impl Item for Weapon {
    fn name(&self) -> &String {
        &self.name
    }

    fn display_name(&self) -> String {
        get_rarity_text_color(&self.rarity, &self.name)
    }

    fn item_type(&self) -> ItemType {
        ItemType::Weapon
    }

    fn display_info(&self) -> String {
        let mut stats = String::new();

        stats.push_str(&format!(
            "{} - ({} {})\n",
            self.display_name(),
            &self.rarity.to_string(),
            &self.item_type().to_string(),
        ));

        if self.power > 0 {
            stats.push_str(&format!("- Power: {}\n", self.power));
        }
        if self.attack_speed > 0 {
            stats.push_str(&format!("- Attack Speed: {}\n", self.attack_speed));
        }
        if self.crit_multiplier > 0 {
            stats.push_str(&format!("- Crit Multiplier: {}\n", self.crit_multiplier));
        }
        if self.crit_chance > 0 {
            stats.push_str(&format!("- Crit Chance: {}\n", self.crit_chance));
        }

        format!("\n{}", stats.trim())
    }
}
