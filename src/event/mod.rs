pub mod battle_event;
pub mod event_loop;
pub mod event_response;
pub mod reward_event;
pub mod travel_event;
pub mod visit_event;

use crate::{
    actions::{Action, ActionType},
    characters::player::Player,
    game_state::GameState,
};

use self::event_loop::EventLoop;
use self::event_response::EventResponse;

pub trait Event {
    fn prompt(&self) -> String;
    fn actions(&self) -> Vec<Action>;
    fn handle_action(
        &mut self,
        search: &str,
        action_type: ActionType,
        game_state: &mut GameState,
        player: &mut Player,
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

    // fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop<Self, EventType = Self>>;
    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop>;
}
