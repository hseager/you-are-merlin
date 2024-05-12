use crate::{
    config::{REWARD_CHANCE_BATTLE, REWARD_CHANCE_CHEST, REWARD_CHANCE_QUEST},
    utilities::roll,
};

use super::ItemRarity;

#[derive(Clone, Debug)]
pub enum RewardType {
    BattleReward,
    ChestReward,
    QuestReward,
}

impl RewardType {
    pub fn roll_rarity(&self) -> ItemRarity {
        let chance = roll();

        let reward_chance = match self {
            RewardType::BattleReward => REWARD_CHANCE_BATTLE,
            RewardType::ChestReward => REWARD_CHANCE_CHEST,
            RewardType::QuestReward => REWARD_CHANCE_QUEST,
        };

        if chance < reward_chance.0 {
            ItemRarity::Common
        } else if chance < reward_chance.0 + reward_chance.1 {
            ItemRarity::Rare
        } else if chance < reward_chance.0 + reward_chance.1 + reward_chance.2 {
            ItemRarity::Epic
        } else {
            ItemRarity::Legendary
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
