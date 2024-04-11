use crate::event::Event;

pub enum EventLoopResponse {
    InProgress(String),
    Complete(String, Box<dyn Event>),
}
