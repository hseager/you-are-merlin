use colored::ColoredString;

use crate::{
    characters::Enemy,
    encounter::{Encounter, Quest},
    world::World,
};

// TODO change PlayerState to return an Instance that contains prompt and actions
// TODO move GameOver and Win into GameState and rename GameState to Game
pub enum PlayerState {
    Travelling,
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
            PlayerState::Travelling => println!("Where would you like to go?"),
            PlayerState::Battle => match world.get_current_encounter() {
                Encounter::Battle(battle) => {
                    let Enemy {
                        name,
                        life,
                        attack,
                        description,
                        ..
                    } = &battle.enemy;

                    println!(
                        "A wild {} appears! (life: {}, attack: {})",
                        name, life, attack
                    );
                    println!("{description}")
                }
                Encounter::BossFight(battle) => {
                    let Enemy {
                        name,
                        life,
                        attack,
                        description,
                        ..
                    } = &battle.enemy;

                    println!("A great danger approaches...");
                    println!("{} (life: {}, attack: {})", name, life, attack);
                    println!("{description}")
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

    // pub fn get_actions(&self) -> () {}
}
