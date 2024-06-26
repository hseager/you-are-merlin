use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

use crate::config::{
    ITEM_GEN_ATTACK_SPEED, ITEM_GEN_CRIT_CHANCE, ITEM_GEN_CRIT_MULTI, ITEM_GEN_POWER,
};
use crate::utilities::round_to_single_decimal;

use super::{get_rarity_property_count, get_rarity_text_color, Item, ItemRarity, ItemType, Stat};

#[derive(Clone)]
pub struct Weapon {
    pub name: String,
    pub rarity: ItemRarity,
    pub power: u16,
    pub attack_speed: u16,
    pub crit_multiplier: f32,
    pub crit_chance: f32,
}

impl Weapon {
    pub fn new(name: String, rarity: ItemRarity) -> Weapon {
        let mut rng = thread_rng();

        let mut properties = [
            Stat::Power,
            Stat::AttackSpeed,
            Stat::CritMultiplier,
            Stat::CritChance,
        ];

        properties.shuffle(&mut rng);

        let selected_properties = properties.iter().take(get_rarity_property_count(&rarity));

        let mut weapon = Weapon {
            name,
            rarity,
            power: 0,
            attack_speed: 0,
            crit_multiplier: 0.0,
            crit_chance: 0.0,
        };

        for property in selected_properties {
            match property {
                Stat::Power => weapon.power = rng.gen_range(ITEM_GEN_POWER.0..=ITEM_GEN_POWER.1),
                Stat::AttackSpeed => {
                    weapon.attack_speed =
                        rng.gen_range(ITEM_GEN_ATTACK_SPEED.0..=ITEM_GEN_ATTACK_SPEED.1)
                }
                Stat::CritMultiplier => {
                    weapon.crit_multiplier = round_to_single_decimal(
                        rng.gen_range(ITEM_GEN_CRIT_MULTI.0..=ITEM_GEN_CRIT_MULTI.1),
                    )
                }
                Stat::CritChance => {
                    weapon.crit_chance = round_to_single_decimal(
                        rng.gen_range(ITEM_GEN_CRIT_CHANCE.0..=ITEM_GEN_CRIT_CHANCE.1),
                    )
                }
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
            "{} - ({} {}) ",
            self.display_name(),
            &self.rarity.to_string(),
            &self.item_type().to_string(),
        ));

        if self.power > 0 {
            stats.push_str(&format!("/ {} Power ", self.power));
        }
        if self.attack_speed > 0 {
            stats.push_str(&format!("/ {} Attack Speed ", self.attack_speed));
        }
        if self.crit_multiplier > 0.0 {
            stats.push_str(&format!("/ {} Crit Multiplier ", self.crit_multiplier));
        }
        if self.crit_chance > 0.0 {
            stats.push_str(&format!("/ {}% Crit Chance ", self.crit_chance));
        }

        stats.trim().to_string()
    }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn power(&self) -> u16 {
        self.power
    }
    fn attack_speed(&self) -> u16 {
        self.attack_speed
    }
    fn crit_multiplier(&self) -> f32 {
        self.crit_multiplier
    }
    fn crit_chance(&self) -> f32 {
        self.crit_chance
    }
}
