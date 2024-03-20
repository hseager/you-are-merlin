use crate::{actions::{get_visiting_actions, Action}, game_data::GameData, player_state::PlayerState};

pub struct GameState<'a>{
    state: PlayerState<'a>,
    current_location: usize,
    current_encounter: usize,
    actions: Vec<Action>,
}

impl<'a> GameState<'a> {
    pub fn new(game_data: &GameData) -> GameState {
        let current_location = 0;
        
        let location = game_data.locations.get(current_location).unwrap();
        
        GameState {
            state: PlayerState::Visiting(&location.name, &location.description),
            current_location,
            current_encounter: 0,
            actions: get_visiting_actions()
        }
    }
    
    pub fn get_prompt(&self) {
        match &self.state {
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

        println!("{}", &self.state.get_actions_display_list());
    }
    
    pub fn handle_action(&mut self, input: &str, game_data: &'a GameData) -> () {
        match input {
            "Travel" => {
                self.state = PlayerState::Travelling(&game_data.locations)
            },
            "Visit" => {
                let current_location = &game_data.locations.get(self.current_location)
                    .expect("Failed to get location");
            
                self.state = PlayerState::Visiting(
                    &current_location.name,
                    &current_location.description,
                );
            },
            _ => println!("Action not found"),
        }
    }
}