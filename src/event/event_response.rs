use super::Event;

pub struct EventResponse {
    pub next_event: Box<dyn Event>,
    pub message: Option<String>,
}

impl EventResponse {
    pub fn new(next_event: Box<dyn Event>, message: Option<String>) -> EventResponse {
        EventResponse {
            next_event,
            message,
        }
    }
}
