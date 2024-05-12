use crate::{text_format::TextFormatter, utilities::roll};

pub mod armour;
pub mod artifact;
pub mod quest_item;
pub mod reward_type;
pub mod weapon;

pub trait Item {
    fn name(&self) -> &String;
    fn display_name(&self) -> String;
    fn item_type(&self) -> ItemType;
    fn display_info(&self) -> String;

    fn clone_box(&self) -> Box<dyn Item>;

    fn power(&self) -> u16 {
        0
    }
    fn attack_speed(&self) -> u16 {
        0
    }
    fn crit_multiplier(&self) -> f32 {
        0.0
    }
    fn crit_chance(&self) -> f32 {
        0.0
    }
    fn max_life(&self) -> i16 {
        0
    }
    fn block(&self) -> u16 {
        0
    }
    fn parry(&self) -> f32 {
        0.0
    }
    fn dodge(&self) -> f32 {
        0.0
    }
}

impl Clone for Box<dyn Item> {
    fn clone(&self) -> Box<dyn Item> {
        self.clone_box()
    }
}

#[derive(Clone, Debug, PartialEq)]
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

pub enum Stat {
    Power,
    AttackSpeed,
    CritMultiplier,
    CritChance,
    MaxLife,
    Block,
    ParryChance,
    DodgeChance,
}

#[derive(Clone)]
pub struct Equipment {
    pub armour: Option<Box<dyn Item>>,
    pub weapon: Option<Box<dyn Item>>,
    pub artifact: Option<Box<dyn Item>>,
}

#[derive(Clone)]
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
    let chance = roll();

    if chance <= 10.0 {
        ItemRarity::Legendary
    } else if chance <= 40.0 {
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
