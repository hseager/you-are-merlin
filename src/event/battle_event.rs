use colored::Colorize;

use crate::{
    actions::{Action, ActionType},
    characters::{enemy::Enemy, player::Player},
    game_data::entities::Battle,
    game_state::GameState,
};

use super::{
    event_loop::{
        battle_event_loop::{BattleEventLoop, Turn},
        EventLoop,
    },
    visit_event::VisitEvent,
    Event, EventResponse,
};

pub struct BattleEvent {
    enemy: Enemy,
    event_loop: BattleEventLoop,
}

impl BattleEvent {
    pub fn new(battle: Battle) -> BattleEvent {
        let event_loop = BattleEventLoop::new(battle.enemy.clone(), Turn::Player);

        BattleEvent {
            enemy: battle.enemy,
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
        &mut self,
        _search: &str,
        action_type: ActionType,
        game_state: &mut GameState,
        _player: &mut Player,
    ) -> Option<EventResponse> {
        match action_type {
            ActionType::Attack => {
                self.event_loop.is_active = true;
                None
            }
            ActionType::Run => {
                let next_event = Box::new(VisitEvent::new(
                    game_state.get_current_location().clone(),
                    game_state.completed_locations.clone(),
                ));
                Some(EventResponse::new(next_event, None))
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        Some(&mut self.event_loop)
    }
}
