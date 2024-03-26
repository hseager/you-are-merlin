use colored::ColoredString;

use crate::{
    game_data::entities::{Encounter, Quest, SideQuest},
    items::Item,
};

pub fn get_visiting_prompt(location_name: &ColoredString, location_description: &'static str) {
    println!(
        "You are currently visiting {}. {}",
        location_name, location_description
    );
    println!("What would you like to do?")
}

pub fn get_travelling_prompt() {
    println!("Where would you like to go?")
}

pub fn get_battle_prompt(encounter: &Encounter) {
    match encounter {
        Encounter::Battle(battle) => {
            println!(
                "A wild {} appears! (life: {}, attack: {})",
                battle.enemy.name, battle.enemy.life, battle.enemy.attack
            );
            println!("{}", battle.enemy.description)
        }
        Encounter::BossFight(battle) => {
            println!("A great danger approaches...");
            println!("{} (life: {}, attack: {})", battle.enemy.name, battle.enemy.life, battle.enemy.attack);
            println!("{}", battle.enemy.description)
        }
        Encounter::Quest(_) => panic!("Shouldn't be possible to battle during a quest.. Unless you're trying to fight an ally?"),
    }
}

pub fn get_quest_prompt(quest: &Quest, accepted_quests: &[&SideQuest]) {
    match quest {
        Quest::MainQuest(quest) => {
            println!(
                "You find a calm area. {} wants to ask you something.",
                &quest.character
            );
            println!(
                "\"{} is in great danger... {} seeks the destruction of this world... They must be stopped...\"",
                &quest.world_name, &quest.boss_name,
            )
        }
        Quest::SideQuest(side_quest) => {
            println!(
                "You find a calm area. {} wants to ask you something.",
                &side_quest.character
            );

            if side_quest.is_accepted(accepted_quests) {
                println!(
                    "\"Do you have it? Please, bring me {} back from {}.\"",
                    &side_quest.item, &side_quest.location_name
                )
            } else {
                println!(
                    "\"Will you find {} from {} and bring it back to me? I will make it worth your while!\"",
                    &side_quest.item, &side_quest.location_name
                )
            }
        }
    }
}

pub fn get_treasure_prompt(item: &Item) {
    println!(
        "You recieve {}! Your attack power increases by {}, and your maximum life grows by {} points.",
        item.name, item.attack, item.max_life
    );
}
