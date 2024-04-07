pub mod battle_event_loop;

pub trait EventLoop {
    fn is_event_loop_active(&self) -> bool;
    fn progress_event_loop(&mut self) -> String;
}
