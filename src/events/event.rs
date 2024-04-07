use crate::{
    actions::{Action, ActionType},
    game_state::GameState,
};

use super::event_loop::event_loop::EventLoop;

pub trait Event {
    fn prompt(&self) -> String;
    fn actions(&self) -> Vec<Action>;
    fn handle_action(
        &self,
        search: &str,
        action_type: ActionType,
        game_state: &mut GameState,
    ) -> Box<dyn Event>;

    fn get_event_loop(&self) -> Option<Box<dyn EventLoop>>;

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
}
