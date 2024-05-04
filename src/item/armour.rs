use crate::config::ITEM_GEN_PARRY_CHANCE;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

use crate::config::{ITEM_GEN_BLOCK, ITEM_GEN_DODGE_CHANCE, ITEM_GEN_MAX_LIFE};

use super::{
    get_rarity_property_count, get_rarity_text_color, get_reward_item_rarity, Item, ItemRarity,
    ItemStat, ItemType,
};

#[derive(Clone)]
pub struct Armour {
    pub name: String,
    pub rarity: ItemRarity,
    pub max_life: i16,
    pub block: u16,
    pub parry_chance: f32,
    pub dodge_chance: f32,
}

impl Armour {
    pub fn new(name: String) -> Armour {
        let mut rng = thread_rng();

        let mut properties = [
            ItemStat::MaxLife,
            ItemStat::Block,
            ItemStat::ParryChance,
            ItemStat::DodgeChance,
        ];

        properties.shuffle(&mut rng);

        let rarity = get_reward_item_rarity();

        let selected_properties = properties.iter().take(get_rarity_property_count(&rarity));

        let mut armour = Armour {
            name,
            rarity,
            max_life: 0,
            block: 0,
            parry_chance: 0.0,
            dodge_chance: 0.0,
        };

        for property in selected_properties {
            match property {
                ItemStat::MaxLife => {
                    armour.max_life = rng.gen_range(ITEM_GEN_MAX_LIFE.0..=ITEM_GEN_MAX_LIFE.1)
                }
                ItemStat::Block => {
                    armour.block = rng.gen_range(ITEM_GEN_BLOCK.0..=ITEM_GEN_BLOCK.1)
                }
                ItemStat::ParryChance => {
                    armour.parry_chance =
                        rng.gen_range(ITEM_GEN_PARRY_CHANCE.0..=ITEM_GEN_PARRY_CHANCE.1)
                }
                ItemStat::DodgeChance => {
                    armour.dodge_chance =
                        rng.gen_range(ITEM_GEN_DODGE_CHANCE.0..=ITEM_GEN_DODGE_CHANCE.1)
                }
                _ => unreachable!(),
            }
        }

        armour
    }
}

impl Item for Armour {
    fn name(&self) -> &String {
        &self.name
    }

    fn display_name(&self) -> String {
        get_rarity_text_color(&self.rarity, &self.name)
    }

    fn item_type(&self) -> ItemType {
        ItemType::Armour
    }

    fn display_info(&self) -> String {
        let mut stats = String::new();

        stats.push_str(&format!(
            "{} - ({} {}) ",
            self.display_name(),
            &self.rarity.to_string(),
            &self.item_type().to_string(),
        ));

        if self.max_life > 0 {
            stats.push_str(&format!("/ {} Max Life ", self.max_life));
        }
        if self.block > 0 {
            stats.push_str(&format!("/ {} Block ", self.block));
        }
        if self.parry_chance > 0.0 {
            stats.push_str(&format!("/ {}% Parry Chance ", self.parry_chance));
        }
        if self.dodge_chance > 0.0 {
            stats.push_str(&format!("/ {}% Dodge Chance ", self.dodge_chance));
        }

        stats.trim().to_string()
    }

    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }

    fn max_life(&self) -> i16 {
        self.max_life
    }

    fn block(&self) -> u16 {
        self.block
    }
}
