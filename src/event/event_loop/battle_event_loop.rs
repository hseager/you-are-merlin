use crate::{
    characters::{enemy::Enemy, fighter::Fighter, player::Player},
    config::BATTLE_INTERVAL_SECONDS,
    event::{battle_event::BattleEvent, travel_event::TravelEvent},
    game_data::entities::Encounter,
    game_state::GameState,
};

use super::{event_loop_response::EventLoopResponse, EventLoop};

#[derive(Clone)]
pub enum Turn {
    Player,
    Enemy,
}

#[derive(Clone)]
pub struct BattleEventLoop {
    pub is_active: bool,
    pub enemy: Enemy,
    pub attack_turn: Turn,
}

impl BattleEventLoop {
    pub fn new(enemy: Enemy, attack_turn: Turn) -> BattleEventLoop {
        BattleEventLoop {
            is_active: false,
            enemy,
            attack_turn,
        }
    }

    pub fn handle_battle_success(
        &mut self,
        game_state: &mut GameState,
        message: String,
    ) -> EventLoopResponse {
        self.is_active = false;

        match game_state.go_to_next_encounter() {
            Encounter::Battle(battle) => {
                EventLoopResponse::Complete(message, Box::new(BattleEvent::new(battle.clone())))
            }
            Encounter::Quest(_) => EventLoopResponse::Complete(
                message,
                Box::new(TravelEvent::new(game_state.get_locations())),
            ),
            Encounter::BossFight(_) => EventLoopResponse::Complete(
                message,
                Box::new(TravelEvent::new(game_state.get_locations())),
            ),
        }

        // let next_encounter = self.current_encounter + 1;
        // let location = self.get_current_location();

        // if next_encounter < location.encounters.len() {
        //     self.current_encounter = next_encounter;

        //     let encounter = self.get_current_encounter();
        //     match encounter {
        //         Encounter::Battle(battle) => {
        //             self.current_event = Box::new(BattleEvent::new(battle.clone()));
        //         }
        //         Encounter::BossFight(_) => {}
        //         //Encounter::Quest(quest) => self.state = PlayerState::Quest(quest.clone()),
        //         Encounter::Quest(quest) => {}
        //     }
        //     None
        // } else {
        //     self.handle_end_of_encounters(location.clone(), player)
        // }
    }

    // pub fn handle_battle_fail(&mut self) {
    //     self.is_active = false;
    // }
}

impl EventLoop for BattleEventLoop {
    fn get_event_loop_interval(&self) -> u64 {
        BATTLE_INTERVAL_SECONDS
    }

    fn is_event_loop_active(&self) -> bool {
        self.is_active
    }

    fn progress_event_loop(
        &mut self,
        player: &mut Player,
        game_state: &mut GameState,
    ) -> EventLoopResponse {
        let mut result = String::new();
        let enemy = &mut self.enemy;

        match self.attack_turn {
            Turn::Player => {
                result = player.attack(enemy);

                if enemy.is_alive() {
                    self.attack_turn = Turn::Enemy;
                } else {
                    self.is_active = false;
                    result = format!("You defeated {}!", enemy.name);
                    // self.handle_battle_success(game_state, current_event);
                    // if let Some(reward_text) = self.game_state.go_to_next_encounter(player) {
                    //     result = format!("{}\n{}", result, reward_text);
                    // }

                    return self.handle_battle_success(game_state, result);
                }
                // result
                EventLoopResponse::InProgress(result)
            }
            Turn::Enemy => {
                result = enemy.attack(player);

                if player.is_alive() {
                    self.attack_turn = Turn::Player;
                } else {
                    self.is_active = false;
                    // self.game_state.state = PlayerState::GameOver;
                    result = format!("{} died!\nGame Over...", player.name);
                    // self.handle_battle_fail();
                }
                EventLoopResponse::InProgress(result)
            }
        }
    }
}
