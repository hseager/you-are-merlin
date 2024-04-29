use crate::{
    actions::{Action, ActionType},
    characters::player::Player,
    game_state::GameState,
    text_format::TextFormatter,
};

use super::{event_loop::EventLoop, event_response::EventResponse, visit_event::VisitEvent, Event};

pub struct ManageEvent {
    player: Player,
}

impl ManageEvent {
    pub fn new(player: Player) -> ManageEvent {
        ManageEvent { player }
    }
}

impl Event for ManageEvent {
    fn prompt(&self) -> Option<String> {
        Some(String::from("You have 10 life"))
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
