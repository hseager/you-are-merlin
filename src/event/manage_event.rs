use crate::{
    actions::{Action, ActionType},
    characters::{
        player::Player,
        stats::{DisplayStats, Stats},
    },
    game_state::GameState,
    text_format::TextFormatter,
};

use super::{event_loop::EventLoop, event_response::EventResponse, visit_event::VisitEvent, Event};

pub struct ManageEvent {
    stats: Stats,
}

impl ManageEvent {
    pub fn new(stats: Stats) -> ManageEvent {
        ManageEvent { stats }
    }
}

impl Event for ManageEvent {
    fn prompt(&self) -> Option<String> {
        let stats = format!(
            "You have {} life, {} power and {} attacks per second.",
            self.stats.life,
            self.stats.power,
            self.stats.display_attacks_per_second()
        );

        Some(stats)
    }
    fn actions(&self) -> Vec<Action> {
        vec![Action::new(ActionType::Continue, "Continue".text_green())]
    }
    fn handle_action(
        &mut self,
        action: Action,
        game_state: &mut GameState,
        _player: &mut Player,
    ) -> EventResponse {
        match action.class {
            ActionType::Continue => {
                let next_event = VisitEvent::build(game_state);
                EventResponse::new(Some(next_event), None)
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}
