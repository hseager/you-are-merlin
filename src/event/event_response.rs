use super::Event;

pub struct EventResponse {
    pub next_event: Box<dyn Event>,
}

impl EventResponse {
    pub fn new(next_event: Box<dyn Event>) -> EventResponse {
        EventResponse { next_event }
    }
}
