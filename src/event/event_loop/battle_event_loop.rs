use crate::{
    characters::{enemy::Enemy, fighter::Fighter, player::Player},
    config::BATTLE_INTERVAL_SECONDS,
    event::battle_event::BattleEvent,
};

use super::EventLoop;

#[derive(Clone)]
pub enum Turn {
    Player,
    Enemy,
}

#[derive(Clone)]
pub struct BattleEventLoop {
    pub enemy: Enemy,
    pub attack_turn: Turn,
}

impl EventLoop for BattleEventLoop {
    type EventType = BattleEvent;

    fn get_event_loop_interval(&self) -> u64 {
        BATTLE_INTERVAL_SECONDS
    }

    fn is_event_loop_active(&self) -> bool {
        false
    }

    fn progress_event_loop(&mut self, player: &mut Player, event: BattleEvent) -> String {
        let mut result = String::new();
        let enemy = &mut self.enemy;

        match self.attack_turn {
            Turn::Player => {
                result = player.attack(enemy);

                if !enemy.is_alive() {
                    result = format!("You defeated {}!", enemy.name);
                    // if let Some(reward_text) = self.game_state.go_to_next_encounter(player) {
                    //     result = format!("{}\n{}", result, reward_text);
                    // }

                    self.attack_turn = Turn::Player;
                } else {
                    self.attack_turn = Turn::Enemy;
                }
                result
            }
            Turn::Enemy => {
                result = enemy.attack(player);

                if player.is_alive() {
                    // self.game_state.state = PlayerState::GameOver;
                    result = format!("{} died!\nGame Over...", player.name);
                } else {
                    self.attack_turn = Turn::Player;
                }
                result
            }
        }
    }
}
