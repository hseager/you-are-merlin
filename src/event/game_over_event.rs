use crate::{
    actions::{Action, ActionType},
    characters::player::Player,
    game_state::GameState,
};

use super::{event_loop::EventLoop, Event, EventResponse};

// An empty event to assign to the EventResponse::Complete when a game ends
pub struct GameOverEvent {}

impl Event for GameOverEvent {
    fn prompt(&self) -> Option<String> {
        None
    }

    fn actions(&self) -> Vec<Action> {
        Vec::new()
    }

    fn handle_action(
        &mut self,
        _search: &str,
        _action_type: ActionType,
        _game_state: &mut GameState,
        _player: &mut Player,
    ) -> EventResponse {
        EventResponse::new(None, None)
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}
