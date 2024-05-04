use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};

use crate::config::{
    ITEM_GEN_ATTACK_SPEED, ITEM_GEN_DODGE_CHANCE, ITEM_GEN_MAX_LIFE, ITEM_GEN_POWER,
};
use crate::utilities::round_to_single_decimal;

use super::{
    get_rarity_property_count, get_rarity_text_color, get_reward_item_rarity, Item, ItemRarity,
    ItemStat, ItemType,
};

// TODO Change these to more interesting stats
#[derive(Clone)]
pub struct Artifact {
    pub name: String,
    pub rarity: ItemRarity,
    pub power: u16,
    pub attack_speed: u16,
    pub max_life: i16,
    pub dodge: f32,
}

impl Artifact {
    pub fn new(name: String) -> Artifact {
        let mut rng = thread_rng();

        let mut properties = [
            ItemStat::Power,
            ItemStat::AttackSpeed,
            ItemStat::MaxLife,
            ItemStat::DodgeChance,
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
            dodge: 0.0,
        };

        for property in selected_properties {
            match property {
                ItemStat::Power => {
                    artifact.power = rng.gen_range(ITEM_GEN_POWER.0..=ITEM_GEN_POWER.1)
                }
                ItemStat::AttackSpeed => {
                    artifact.attack_speed =
                        rng.gen_range(ITEM_GEN_ATTACK_SPEED.0..=ITEM_GEN_ATTACK_SPEED.1)
                }
                ItemStat::MaxLife => {
                    artifact.max_life = rng.gen_range(ITEM_GEN_MAX_LIFE.0..=ITEM_GEN_MAX_LIFE.1)
                }
                ItemStat::DodgeChance => {
                    artifact.dodge = round_to_single_decimal(
                        rng.gen_range(ITEM_GEN_DODGE_CHANCE.0..=ITEM_GEN_DODGE_CHANCE.1),
                    )
                }
                _ => unreachable!(),
            }
        }

        artifact
    }
}

impl Item for Artifact {
    fn name(&self) -> &String {
        &self.name
    }

    fn display_name(&self) -> String {
        get_rarity_text_color(&self.rarity, &self.name)
    }

    fn item_type(&self) -> ItemType {
        ItemType::Artifact
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
        if self.max_life > 0 {
            stats.push_str(&format!("/ {} Max Life ", self.max_life));
        }
        if self.dodge > 0.0 {
            stats.push_str(&format!("/ {}% Dodge ", self.dodge));
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
    fn max_life(&self) -> i16 {
        self.max_life
    }
    fn dodge(&self) -> f32 {
        self.dodge
    }
}
