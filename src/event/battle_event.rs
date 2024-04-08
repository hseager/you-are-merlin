use colored::Colorize;

use crate::{
    actions::{Action, ActionType},
    characters::enemy::Enemy,
    game_data::entities::Battle,
    game_state::GameState,
};

use super::{
    event_loop::{
        battle_event_loop::{BattleEventLoop, Turn},
        EventLoop,
    },
    visit_event::VisitEvent,
    Event, EventResponse, EventType,
};

pub enum BattleState {
    Identifing,
    Fighting,
}

pub struct BattleEvent {
    enemy: Enemy,
    pub state: BattleState,
    event_loop: BattleEventLoop,
}

impl BattleEvent {
    pub fn new(battle: Battle) -> BattleEvent {
        let event_loop = BattleEventLoop {
            attack_turn: Turn::Player,
            enemy: battle.enemy.clone(),
        };

        BattleEvent {
            enemy: battle.enemy,
            state: BattleState::Identifing,
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
    ) -> Option<EventResponse> {
        match action_type {
            ActionType::Attack => {
                self.state = BattleState::Fighting;
                None
            }
            ActionType::Run => Some(EventResponse {
                next_event: EventType::VisitEvent(VisitEvent::new(
                    game_state.get_current_location().clone(),
                    game_state.completed_locations.clone(),
                )),
            }),
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop<EventType = Self>> {
        Some(&mut self.event_loop)
    }

    // fn progress_event_loop(&self) {
    //     self.event_loop.progress_event_loop(player, self);
    // }
}
