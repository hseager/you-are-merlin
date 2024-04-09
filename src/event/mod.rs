pub mod battle_event;
pub mod event_loop;
pub mod travel_event;
pub mod visit_event;

use crate::{
    actions::{Action, ActionType},
    game_state::GameState,
};

use crate::event::event_loop::EventLoop;

pub struct EventResponse {
    pub next_event: Box<dyn Event>,
}

pub trait Event {
    fn prompt(&self) -> String;
    fn actions(&self) -> Vec<Action>;
    fn handle_action(
        &mut self,
        search: &str,
        action_type: ActionType,
        game_state: &mut GameState,
    ) -> Option<EventResponse>;

    fn find_action(&self, search: &str) -> Option<Action> {
        self.actions()
            .iter()
            .find(|action| {
                action
                    .name
                    .trim()
                    .to_lowercase()
                    .contains(&search.to_lowercase())
            })
            .cloned()
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop<Self, EventType = Self>>;
}
