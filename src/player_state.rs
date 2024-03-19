use colored::ColoredString;

use crate::{
    actions::*,
    encounter::{Encounter, Quest},
    location::Location,
    world::World,
};

// TODO move GameOver and Win into GameState and rename GameState to Game
// TODO Lose coupling of world
pub enum PlayerState {
    Travelling(Vec<Location>),
    Visiting(ColoredString, &'static str),
    Battle,
    Quest,
    GameOver,
    Win,
}

impl PlayerState {
    pub fn get_prompt(&self, world: &World) -> () {
        match &self {
            PlayerState::Visiting(location_name, location_description) => {
                println!(
                    "You are currently visiting {}. {}",
                    location_name,
                    location_description
                );
                println!("What would you like to do?")
            }
            PlayerState::Travelling(_) => println!("Where would you like to go?"),
            PlayerState::Battle => match world.get_current_encounter() {
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
            },
            PlayerState::Quest => match world.get_current_encounter() {
                Encounter::Quest(quest) => match quest {
                    Quest::MainQuest(quest) => {
                        println!(
                            "You find a calm area. {} wants to ask you something.",
                            quest.character
                        );
                        println!(
                            "\"{} is in great danger... {} seeks the destruction of this world... They must be stopped...\"",
                            world.name, quest.boss_name,
                        )
                    }
                    Quest::SideQuest(quest) => {
                        println!(
                            "You find a calm area. {} wants to ask you something.",
                            quest.character
                        );
                        println!(
                            "\"Will you find {} and bring it back to me? I will make it worth your while...\"",
                            quest.item
                        )
                    }
                },
                _ => panic!("Encounter not a quest when playerState is a quest.")
            },
            _ => panic!("Unhandled PlayerState")
        }
    }

    pub fn get_actions(&self) -> Vec<Action> {
        match self {
            PlayerState::Visiting(_location_name, _location_description) => get_visiting_actions(),
            PlayerState::Battle => get_battle_actions(),
            PlayerState::Quest => get_quest_actions(),
            PlayerState::Travelling(locations) => get_locations_as_actions(locations),
            _ => vec![],
        }
    }

    pub fn get_actions_display_list(&self) -> String {
        self.get_actions()
            .iter()
            .map(|action| action.name.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}
