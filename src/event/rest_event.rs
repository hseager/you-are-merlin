use crate::{actions::Action, characters::player::Player, game_state::GameState};

use super::{
    event_loop::{rest_event_loop::RestEventLoop, EventLoop},
    Event, EventResponse,
};

pub struct RestEvent {
    event_loop: RestEventLoop,
}

impl RestEvent {
    pub fn new() -> RestEvent {
        let event_loop = RestEventLoop::new();

        RestEvent { event_loop }
    }
}

impl Event for RestEvent {
    fn prompt(&self) -> Option<String> {
        Some(String::from("You stay and rest a while..."))
    }

    fn actions(&self) -> Vec<Action> {
        Vec::new()
    }

    fn handle_action(
        &mut self,
        _action: Action,
        _game_state: &mut GameState,
        _player: &mut Player,
    ) -> EventResponse {
        EventResponse::new(None, None)
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        Some(&mut self.event_loop)
    }
}
