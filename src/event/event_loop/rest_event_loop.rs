use crate::{
    characters::player::Player, config::REST_INTERVAL_SECONDS, event::visit_event::VisitEvent,
    game_state::GameState,
};

use super::{event_loop_response::EventLoopResponse, EventLoop};

pub struct RestEventLoop {
    pub is_active: bool,
}

impl RestEventLoop {
    pub fn new() -> RestEventLoop {
        RestEventLoop { is_active: true }
    }
}

impl EventLoop for RestEventLoop {
    fn get_event_loop_interval(&self) -> u64 {
        REST_INTERVAL_SECONDS
    }

    fn is_event_loop_active(&self) -> bool {
        self.is_active
    }

    fn progress_event_loop(
        &mut self,
        player: &mut Player,
        game_state: &mut GameState,
    ) -> EventLoopResponse {
        if player.life < player.max_life {
            EventLoopResponse::InProgress(player.heal())
        } else {
            let next_event = Box::new(VisitEvent::new(
                game_state.get_current_location().clone(),
                game_state.completed_locations.clone(),
            ));
            EventLoopResponse::Complete("You fully recover your health.".to_string(), next_event)
        }
    }
}
