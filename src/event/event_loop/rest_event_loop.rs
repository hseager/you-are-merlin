use crate::{
    characters::{fighter::Fighter, player::Player},
    config::REST_INTERVAL_MILLIS,
    event::visit_event::VisitEvent,
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
    fn get_event_loop_interval(&self) -> u32 {
        REST_INTERVAL_MILLIS
    }

    fn is_event_loop_active(&self) -> bool {
        self.is_active
    }

    fn progress_event_loop(
        &mut self,
        _current_epoch_milli: u64,
        player: &mut Player,
        game_state: &mut GameState,
    ) -> EventLoopResponse {
        if player.stats.life < player.max_life() {
            EventLoopResponse::InProgress(Some(player.rest()))
        } else {
            EventLoopResponse::Complete(
                String::from("You fully recover your health."),
                VisitEvent::build(game_state),
            )
        }
    }
}
