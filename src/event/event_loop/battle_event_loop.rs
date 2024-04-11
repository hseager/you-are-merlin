use crate::{
    characters::{enemy::Enemy, fighter::Fighter, player::Player},
    config::BATTLE_INTERVAL_SECONDS,
};

use super::EventLoop;

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

    pub fn handle_battle_success(&self) {}

    pub fn handle_battle_fail(&self) {}
}

impl EventLoop for BattleEventLoop {
    fn get_event_loop_interval(&self) -> u64 {
        BATTLE_INTERVAL_SECONDS
    }

    fn is_event_loop_active(&self) -> bool {
        self.is_active
    }

    fn progress_event_loop(&mut self, player: &mut Player) -> String {
        let mut result = String::new();
        let enemy = &mut self.enemy;

        match self.attack_turn {
            Turn::Player => {
                result = player.attack(enemy);

                if enemy.is_alive() {
                    self.attack_turn = Turn::Enemy;
                } else {
                    result = format!("You defeated {}!", enemy.name);
                    self.is_active = false;
                    self.handle_battle_success();
                    // if let Some(reward_text) = self.game_state.go_to_next_encounter(player) {
                    //     result = format!("{}\n{}", result, reward_text);
                    // }
                }
                result
            }
            Turn::Enemy => {
                result = enemy.attack(player);

                if player.is_alive() {
                    self.attack_turn = Turn::Player;
                } else {
                    // self.game_state.state = PlayerState::GameOver;
                    result = format!("{} died!\nGame Over...", player.name);
                    self.is_active = false;
                }
                result
            }
        }
    }
}
