use crate::{
    actions::{Action, ActionType},
    game_data::entities::Location,
    game_state::GameState,
};

use super::{event::Event, event_loop::event_loop::EventLoop, visit_event::VisitEvent};

pub struct TravelEvent {
    locations: Vec<Location>,
}

impl TravelEvent {
    pub fn new(locations: Vec<Location>) -> TravelEvent {
        TravelEvent { locations }
    }
}

impl Event for TravelEvent {
    fn prompt(&self) -> String {
        "Where would you like to go?".to_string()
    }

    fn actions(&self) -> Vec<Action> {
        self.locations
            .iter()
            .map(|location| Action::new(ActionType::MoveToLocation, location.name.to_owned()))
            .collect()
    }

    fn handle_action(
        &self,
        search: &str,
        action_type: ActionType,
        game_state: &mut GameState,
    ) -> Box<dyn Event> {
        match action_type {
            ActionType::MoveToLocation => {
                game_state.change_location_by_name(search.to_string());

                Box::new(VisitEvent::new(
                    game_state.get_current_location().clone(),
                    game_state.completed_locations.clone(),
                ))
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&self) -> Option<Box<dyn EventLoop>> {
        None
    }
}
