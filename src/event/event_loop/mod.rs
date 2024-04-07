use crate::characters::player::Player;

pub mod battle_event_loop;

pub trait EventLoop {
    type EventType;

    fn is_event_loop_active(&self) -> bool;
    fn get_event_loop_interval(&self) -> u64;
    fn progress_event_loop(&mut self, player: &mut Player, event: Self::EventType) -> String;
}
