use crate::utilities::roll;

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

        // TODO move these to config as tuple
        match self {
            RewardType::BattleReward => {
                if chance <= 10.0 {
                    ItemRarity::Epic
                } else if chance <= 40.0 {
                    ItemRarity::Rare
                } else {
                    ItemRarity::Common
                }
            }
            RewardType::ChestReward => {
                if chance <= 10.0 {
                    ItemRarity::Legendary
                } else if chance <= 40.0 {
                    ItemRarity::Epic
                } else {
                    ItemRarity::Rare
                }
            }
            RewardType::QuestReward => {
                if chance <= 10.0 {
                    ItemRarity::Legendary
                } else if chance <= 40.0 {
                    ItemRarity::Epic
                } else {
                    ItemRarity::Rare
                }
            }
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
