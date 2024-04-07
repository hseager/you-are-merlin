use colored::Colorize;

use crate::{
    actions::{Action, ActionType},
    characters::{enemy::Enemy, fighter::Fighter, player::Player},
    config::BATTLE_INTERVAL_SECONDS,
    game_data::entities::Battle,
    game_state::GameState,
};

use super::{
    event::Event,
    event_loop::{battle_event_loop::BattleEventLoop, event_loop::EventLoop},
    travel_event::TravelEvent,
    visit_event::VisitEvent,
};

enum Turn {
    Player,
    Enemy,
}

pub struct BattleEvent {
    enemy: Enemy,
    attack_turn: Turn,
    event_loop: BattleEventLoop,
}

impl BattleEvent {
    pub fn new(battle: Battle) -> BattleEvent {
        let event_loop = BattleEventLoop {
            interval: BATTLE_INTERVAL_SECONDS,
        };

        BattleEvent {
            enemy: battle.enemy,
            attack_turn: Turn::Player,
            event_loop,
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

    fn get_event_loop(&self) -> Option<Box<dyn EventLoop>> {
        Some(Box::new(self.event_loop.clone()))
    }
}
