use crate::{
    actions::{Action, ActionType},
    characters::player::Player,
    game_data::entities::Location,
    game_state::GameState,
};

use super::{event_loop::EventLoop, visit_event::VisitEvent, Event, EventResponse};

pub struct TravelEvent {
    locations: Vec<Location>,
}

impl TravelEvent {
    pub fn new(locations: Vec<Location>) -> TravelEvent {
        TravelEvent { locations }
    }
}

impl Event for TravelEvent {
    fn prompt(&self) -> Option<String> {
        Some("Where would you like to go?".to_string())
    }

    fn actions(&self) -> Vec<Action> {
        self.locations
            .iter()
            .map(|location| Action::new(ActionType::MoveToLocation, location.name.to_owned()))
            .collect()
    }

    fn handle_action(
        &mut self,
        action: Action,
        game_state: &mut GameState,
        _player: &mut Player,
    ) -> EventResponse {
        match action.class {
            ActionType::MoveToLocation => {
                game_state.change_location_by_name(action.name);

                let next_event = Box::new(VisitEvent::new(
                    game_state.get_current_location().clone(),
                    game_state.completed_locations.clone(),
                ));

                EventResponse::new(Some(next_event), None)
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}
