use crate::characters::player::Player;

use super::event::Event;

pub trait EventLoop: Event {
    fn is_event_loop_active(&self) -> bool;
    fn handle_event_loop(&mut self, player: &mut Player) -> String;
    fn get_event_loop_interval(&self) -> u64;
}
