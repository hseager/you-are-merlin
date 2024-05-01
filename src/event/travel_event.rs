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
        Some(String::from("Where would you like to go?"))
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

                EventResponse::new(Some(VisitEvent::build(game_state)), None)
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}
