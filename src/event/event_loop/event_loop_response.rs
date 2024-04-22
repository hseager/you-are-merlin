use crate::event::Event;

pub enum EventLoopResponse {
    InProgress(Option<String>),
    Complete(String, Box<dyn Event>),
}
