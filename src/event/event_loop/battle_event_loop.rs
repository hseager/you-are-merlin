use crate::{
    characters::{enemy::Enemy, fighter::Fighter, player::Player},
    config::{
        BATTLE_BASE_CHANCE_TO_FIND_ITEM, BATTLE_CHANCE_TO_FIND_ITEM_DECAY, BATTLE_INTERVAL_MILLIS,
    },
    event::{
        battle_event::BattleEvent, boss_fight_event::BossFightEvent,
        game_over_event::GameOverEvent, reward_event::RewardEvent,
    },
    game_data::entities::{Encounter, LocationType},
    game_state::GameState,
    item::{quest_item::QuestItem, reward_type::RewardType},
    text_format::TextFormatter,
    utilities::roll,
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
        player: &mut Player,
        game_state: &mut GameState,
        message: String,
    ) -> EventLoopResponse {
        self.is_active = false;

        match game_state.get_current_encounter() {
            Encounter::Battle(_) => self.handle_normal_battle_success(player, game_state, message),
            Encounter::BossFight(_) => self.handle_boss_battle_success(game_state),
            _ => panic!("Unhandled encounter at the end of a battle loop."),
        }
    }

    fn handle_normal_battle_success(
        &self,
        player: &mut Player,
        game_state: &mut GameState,
        mut message: String,
    ) -> EventLoopResponse {
        match game_state.go_to_next_encounter() {
            Some(encounter) => match encounter {
                Encounter::Battle(battle) => {
                    let next_battle = battle.clone();

                    if roll() <= self.get_item_find_chance(player) {
                        message = self.handle_find_item(player, game_state, message);
                    }

                    EventLoopResponse::Complete(message, Box::new(BattleEvent::new(next_battle)))
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
                    LocationType::Dungeon(quest_item_name) => EventLoopResponse::Complete(
                        message,
                        Box::new(RewardEvent::new(
                            game_state.get_current_location().clone(),
                            QuestItem::new(String::from(quest_item_name)),
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
                "You defeated {}! {} is saved!\n{}!",
                &self.enemy.name,
                game_state.game_data.world_name,
                "You win".text_green_bold()
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
            format!(
                "{}\n{} died!\n{}...",
                response_text,
                player.name,
                "Game Over".text_red_bold()
            ),
            Box::new(GameOverEvent {}),
        )
    }

    pub fn handle_find_item(
        &self,
        player: &mut Player,
        game_state: &mut GameState,
        message: String,
    ) -> String {
        let item = game_state.get_reward_item(RewardType::BattleReward);
        let message = format!(
            "{}\nYour keen eye catches sight of something amidst the debris of the battle:\n{}",
            message,
            item.display_info()
        );
        player.add_item_to_inventory(item);

        message
    }

    pub fn get_item_find_chance(&self, player: &mut Player) -> f32 {
        // Decrease battle item find chance for each item in the inventory
        let decay = (player.inventory.len() as u16 * BATTLE_CHANCE_TO_FIND_ITEM_DECAY) as f32;

        BATTLE_BASE_CHANCE_TO_FIND_ITEM as f32 - decay
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

        if player.can_attack(current_epoch_milli, self.player_last_attack_time) {
            response_text = player.attack(enemy);

            if !enemy.is_alive() {
                response_text = format!("{}\nYou defeated {}!", response_text, enemy.name);
                return self.handle_battle_success(player, game_state, response_text);
            }

            self.player_last_attack_time = current_epoch_milli;

            return EventLoopResponse::InProgress(Some(response_text));
        }

        if enemy.can_attack(current_epoch_milli, self.enemy_last_attack_time) {
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
