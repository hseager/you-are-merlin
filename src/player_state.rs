use crate::{
    characters::Enemy,
    encounter::{Encounter, Quest},
    game_manager::GameState,
};

pub enum PlayerState {
    Travelling,
    Visiting,
    Battle,
    Quest,
    GameOver,
    Win,
}

impl PlayerState {
    pub fn get_prompt(&self, game_state: &GameState) -> () {
        let GameState { world, .. } = game_state;

        match &self {
            PlayerState::Visiting => {
                if let Some(location) = world.locations.get(world.current_location) {
                    println!(
                        "You are currently visiting {}. {}",
                        location.display_name(),
                        location.description
                    );
                    println!("What would you like to do?")
                } else {
                    panic!("Couldn't find location!");
                }
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
                Encounter::Quest(_) => panic!("Shouldn't be possible to battle during a quest.. Unless we want to fight an ally?"),
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
}
