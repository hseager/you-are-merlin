use colored::ColoredString;

use crate::{
    game_data::entities::{Encounter, Quest, SideQuest},
    items::Item,
};

pub fn get_visiting_prompt(
    location_name: &ColoredString,
    location_description: &'static str,
) -> String {
    format!(
        "You are currently visiting {}. {}\nWhat would you like to do?",
        location_name, location_description
    )
}

pub fn get_travelling_prompt() -> String {
    "Where would you like to go?".to_string()
}

pub fn get_battle_prompt(encounter: &Encounter) -> String {
    match encounter {
        Encounter::Battle(battle) => {
            format!(
                "A wild {} appears! (life: {}, attack: {})\n{}",
                battle.enemy.name, battle.enemy.life, battle.enemy.attack, battle.enemy.description
            )
        }
        Encounter::BossFight(battle) => {
            format!("A great danger approaches...\n{} (life: {}, attack: {})\n{}", battle.enemy.name, battle.enemy.life, battle.enemy.attack, battle.enemy.description)
        }
        Encounter::Quest(_) => panic!("Shouldn't be possible to battle during a quest.. Unless you're trying to fight an ally?"),
    }
}

pub fn get_quest_prompt(quest: &Quest, accepted_quests: &[&SideQuest]) -> String {
    match quest {
        Quest::MainQuest(quest) => {
            format!(
                "You find a calm area. {} wants to ask you something.\n\"{} is in great danger... {} seeks the destruction of this world... They must be stopped...\"",
                &quest.character, &quest.world_name, &quest.boss_name
            )
        }
        Quest::SideQuest(side_quest) => {
            let mut text = String::new();

            if side_quest.is_accepted(accepted_quests) {
                text = format!(
                    "\"Do you have it? Please, bring me {} back from {}.\"",
                    &side_quest.item, &side_quest.location_name
                );
            } else {
                text = format!(
                    "\"Will you find {} from {} and bring it back to me? I will make it worth your while!\"",
                    &side_quest.item, &side_quest.location_name
                )
            }

            format!(
                "You find a calm area. {} wants to ask you something.\n{}",
                &side_quest.character, text
            )
        }
    }
}

pub fn get_treasure_prompt(item: &Item) -> String {
    format!(
        "You recieve {}! Your attack power increases by {}, and your maximum life grows by {} points.",
        item.name, item.attack, item.max_life
    )
}
