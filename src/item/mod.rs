use rand::{thread_rng, Rng};

use self::{armour::Armour, artifact::Artifact, weapon::Weapon};
use crate::text_format::TextFormatter;

pub mod armour;
pub mod artifact;
pub mod item_builder;
pub mod quest_item;
pub mod weapon;

pub trait Item {
    fn name(&self) -> &String;
    fn display_name(&self) -> String;
    fn item_type(&self) -> ItemType;
    fn display_info(&self) -> String;
}

#[derive(Clone, Debug)]
pub enum ItemType {
    Armour,
    Weapon,
    Artifact,
    QuestItem,
}

impl ItemType {
    #![allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            ItemType::Armour => String::from("Armour"),
            ItemType::Weapon => String::from("Weapon"),
            ItemType::Artifact => String::from("Artifact"),
            ItemType::QuestItem => String::from("Quest item"),
        }
    }
}

pub enum ItemStat {
    Power,
    AttackSpeed,
    CritMultiplier,
    CritChance,
    MaxLife,
    Block,
    ParryChance,
    DodgeChance,
}

pub struct Equipment {
    pub armour: Option<Armour>,
    pub weapon: Option<Weapon>,
    pub artifact: Option<Artifact>,
}

pub enum ItemRarity {
    Common,
    Rare,
    Epic,
    Legendary,
}

impl ItemRarity {
    #![allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            ItemRarity::Common => String::from("Common"),
            ItemRarity::Rare => String::from("Rare"),
            ItemRarity::Epic => String::from("Epic"),
            ItemRarity::Legendary => String::from("Legendary"),
        }
    }
}

pub fn get_reward_item_rarity() -> ItemRarity {
    let mut rng = thread_rng();
    let chance: u8 = rng.gen_range(1..=100);

    if chance <= 20 {
        ItemRarity::Legendary
    } else if chance <= 50 {
        ItemRarity::Epic
    } else {
        ItemRarity::Rare
    }
}

pub fn get_rarity_property_count(rarity: &ItemRarity) -> usize {
    match rarity {
        ItemRarity::Common => 1,
        ItemRarity::Rare => 2,
        ItemRarity::Epic => 3,
        ItemRarity::Legendary => 4,
    }
}

pub fn get_rarity_text_color(rarity: &ItemRarity, name: &String) -> String {
    match rarity {
        ItemRarity::Common => name.text_bold(),
        ItemRarity::Rare => name.text_blue_bold(),
        ItemRarity::Epic => name.text_purple_bold(),
        ItemRarity::Legendary => name.text_orange_bold(),
    }
}