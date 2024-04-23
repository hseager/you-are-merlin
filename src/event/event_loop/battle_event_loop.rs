use crate::{
    characters::{enemy::Enemy, fighter::Fighter, player::Player},
    config::BATTLE_INTERVAL_MILLIS,
    event::{
        battle_event::BattleEvent, boss_fight_event::BossFightEvent,
        game_over_event::GameOverEvent, reward_event::RewardEvent,
    },
    game_data::entities::{Encounter, LocationType},
    game_state::GameState,
    text_format::TextFormatter,
};

use super::{event_loop_response::EventLoopResponse, EventLoop};

#[derive(Clone)]
pub struct BattleEventLoop {
    pub is_active: bool,
    pub enemy: Enemy,
    pub player_last_attack_time: i32,
    pub enemy_last_attack_time: i32,
}

impl BattleEventLoop {
    pub fn new(enemy: Enemy) -> BattleEventLoop {
        BattleEventLoop {
            is_active: false,
            enemy,
            player_last_attack_time: 0,
            enemy_last_attack_time: 0,
        }
    }

    pub fn start(&mut self) {
        self.is_active = true;
    }

    pub fn handle_battle_success(
        &mut self,
        game_state: &mut GameState,
        message: String,
    ) -> EventLoopResponse {
        self.is_active = false;

        match game_state.get_current_encounter() {
            Encounter::Battle(_) => self.handle_normal_battle_success(game_state, message),
            Encounter::BossFight(_) => self.handle_boss_battle_success(game_state),
            _ => panic!("Unhandled encounter at the end of a battle loop."),
        }
    }

    fn handle_normal_battle_success(
        &self,
        game_state: &mut GameState,
        message: String,
    ) -> EventLoopResponse {
        match game_state.go_to_next_encounter() {
            Some(encounter) => match encounter {
                Encounter::Battle(battle) => {
                    EventLoopResponse::Complete(message, Box::new(BattleEvent::new(battle.clone())))
                }
                Encounter::BossFight(battle) => EventLoopResponse::Complete(
                    message,
                    Box::new(BossFightEvent::new(battle.clone())),
                ),
                Encounter::Quest(_) => panic!("Not implemented quests after a battle. yet..."),
            },
            None => {
                let current_location = game_state.get_current_location();

                match current_location.class {
                    LocationType::Dungeon(quest_item) => EventLoopResponse::Complete(
                        message,
                        Box::new(RewardEvent::new(
                            game_state.get_current_location().clone(),
                            quest_item.text_bold(),
                        )),
                    ),
                    _ => panic!("Shouldn't be anything apart from a Dungeon at the end of a battle.. For now..."),
                }
            }
        }
    }

    fn handle_boss_battle_success(&self, game_state: &mut GameState) -> EventLoopResponse {
        game_state.is_running = false;

        EventLoopResponse::Complete(
            format!(
                "You defeated {}! {} is saved!\nYou win!",
                &self.enemy.name, game_state.game_data.world_name
            ),
            Box::new(GameOverEvent {}),
        )
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
    fn get_event_loop_interval(&self) -> u32 {
        BATTLE_INTERVAL_MILLIS
    }

    fn is_event_loop_active(&self) -> bool {
        self.is_active
    }

    fn progress_event_loop(
        &mut self,
        current_epoch_milli: i32,
        player: &mut Player,
        game_state: &mut GameState,
    ) -> EventLoopResponse {
        let mut response_text: String;
        let enemy = &mut self.enemy;

        if player_can_attack(
            current_epoch_milli,
            self.player_last_attack_time,
            player.attack_speed,
        ) {
            response_text = player.attack(enemy);

            if !enemy.is_alive() {
                response_text = format!("{}\nYou defeated {}!", response_text, enemy.name);
                return self.handle_battle_success(game_state, response_text);
            }

            self.player_last_attack_time = current_epoch_milli;

            return EventLoopResponse::InProgress(Some(response_text));
        }

        if enemy_can_attack(
            current_epoch_milli,
            self.enemy_last_attack_time,
            enemy.attack_speed,
        ) {
            response_text = enemy.attack(player);

            if !player.is_alive() {
                return self.handle_battle_fail(response_text, game_state, player);
            }

            self.enemy_last_attack_time = current_epoch_milli;

            return EventLoopResponse::InProgress(Some(response_text));
        }

        EventLoopResponse::InProgress(None)
    }
}

fn player_can_attack(
    current_epoch_milli: i32,
    player_last_attack_time: i32,
    player_attack_speed: u16,
) -> bool {
    current_epoch_milli - player_last_attack_time >= player_attack_speed as i32
}

fn enemy_can_attack(
    current_epoch_milli: i32,
    enemy_last_attack_time: i32,
    enemy_attack_speed: u16,
) -> bool {
    current_epoch_milli - enemy_last_attack_time >= enemy_attack_speed as i32
}
