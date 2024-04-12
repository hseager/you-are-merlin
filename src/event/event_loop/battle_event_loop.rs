use colored::Colorize;

use crate::{
    characters::{enemy::Enemy, fighter::Fighter, player::Player},
    config::BATTLE_INTERVAL_SECONDS,
    event::{
        battle_event::BattleEvent, game_over_event::GameOverEvent, reward_event::RewardEvent,
        travel_event::TravelEvent,
    },
    game_data::entities::{Encounter, LocationType},
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

        // TODO other encounters
        match game_state.go_to_next_encounter() {
            Some(encounter) => match encounter {
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
            },
            None => {
                let current_location = game_state.get_current_location();

                match current_location.class {
                    LocationType::Dungeon(quest_item) => EventLoopResponse::Complete(
                        message,
                        Box::new(RewardEvent::new(
                            game_state.get_current_location().clone(),
                            quest_item.bold(),
                        )),
                    ),
                    _ => panic!("Shouldn't be anything apart from a Dungeon at the end of a battle.. For now..."),
                }
            }
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

    pub fn handle_battle_fail(
        &mut self,
        response_text: String,
        game_state: &mut GameState,
        player: &mut Player,
    ) -> EventLoopResponse {
        self.is_active = false;

        game_state.is_running = false;
        EventLoopResponse::Complete(
            format!("{}\n{} died!\nGame Over...", response_text, player.name),
            Box::new(GameOverEvent {}),
        )
    }
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
        let mut response_text: String;
        let enemy = &mut self.enemy;

        match self.attack_turn {
            Turn::Player => {
                response_text = player.attack(enemy);

                if enemy.is_alive() {
                    self.attack_turn = Turn::Enemy;
                } else {
                    response_text = format!("You defeated {}!", enemy.name);
                    return self.handle_battle_success(game_state, response_text);
                }
                EventLoopResponse::InProgress(response_text)
            }
            Turn::Enemy => {
                response_text = enemy.attack(player);

                if player.is_alive() {
                    self.attack_turn = Turn::Player;
                } else {
                    return self.handle_battle_fail(response_text, game_state, player);
                }
                EventLoopResponse::InProgress(response_text)
            }
        }
    }
}
