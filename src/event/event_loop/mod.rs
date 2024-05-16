use crate::{characters::player::Player, game_state::GameState};

use self::event_loop_response::EventLoopResponse;

pub mod battle_event_loop;
pub mod event_loop_response;
pub mod rest_event_loop;

pub trait EventLoop {
    fn is_event_loop_active(&self) -> bool;
    fn get_event_loop_interval(&self) -> u32;
    fn progress_event_loop(
        &mut self,
        current_epoch_milli: u64,
        player: &mut Player,
        game_state: &mut GameState,
    ) -> EventLoopResponse;
}
