use colored::Colorize;

use crate::{
    actions::{Action, ActionType},
    characters::{enemy::Enemy, fighter::Fighter, player::Player},
    config::BATTLE_INTERVAL_SECONDS,
    game_data::entities::Battle,
    game_state::GameState,
};

use super::{
    event::Event, event_loop::EventLoop, travel_event::TravelEvent, visit_event::VisitEvent,
};

enum Turn {
    Player,
    Enemy,
}

pub struct BattleEvent {
    enemy: Enemy,
    attack_turn: Turn,
}

impl BattleEvent {
    pub fn new(battle: Battle) -> BattleEvent {
        BattleEvent {
            enemy: battle.enemy,
            attack_turn: Turn::Player,
        }
    }
}

impl Event for BattleEvent {
    fn prompt(&self) -> String {
        format!(
            "A wild {} appears! (life: {}, attack: {})\n{}",
            &self.enemy.name, &self.enemy.life, &self.enemy.attack, &self.enemy.description
        )
    }

    fn actions(&self) -> Vec<Action> {
        vec![
            Action::new(ActionType::Attack, "Attack".red()),
            Action::new(ActionType::Run, "Run".cyan()),
        ]
    }

    fn handle_action(
        &self,
        _search: &str,
        action_type: ActionType,
        game_state: &mut GameState,
    ) -> Box<dyn Event> {
        match action_type {
            ActionType::Attack => Box::new(TravelEvent::new(game_state.get_locations())),
            ActionType::Run => Box::new(VisitEvent::new(
                game_state.get_current_location().clone(),
                game_state.completed_locations.clone(),
            )),
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl EventLoop for BattleEvent {
    fn get_event_loop_interval(&self) -> u64 {
        BATTLE_INTERVAL_SECONDS
    }

    fn is_event_loop_active(&self) -> bool {
        true
    }

    fn handle_event_loop(&mut self, player: &mut Player) -> String {
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
