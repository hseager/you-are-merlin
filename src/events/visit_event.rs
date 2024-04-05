use colored::Colorize;

use crate::{
    actions::{Action, ActionType},
    game_data::entities::{Location, LocationType},
    game_state::GameState,
};

use super::{event::Event, travel_event::TravelEvent};

pub struct VisitEvent {
    current_location: Location,
    completed_locations: Vec<Location>,
}

impl VisitEvent {
    pub fn new(location: Location, completed_locations: Vec<Location>) -> VisitEvent {
        VisitEvent {
            current_location: location,
            completed_locations,
        }
    }
}

impl Event for VisitEvent {
    fn prompt(&self) -> String {
        format!(
            "You are currently visiting {}. {}\nWhat would you like to do?",
            &self.current_location.name, &self.current_location.description
        )
    }

    fn actions(&self) -> Vec<Action> {
        let mut actions = vec![Action::new(ActionType::Travel, "Travel".yellow())];

        if !self
            .completed_locations
            .iter()
            .any(|l| l.name == self.current_location.name)
        {
            actions.push(Action::new(ActionType::Explore, "Explore".blue()))
        }

        if let LocationType::SafeZone = &self.current_location.class {
            actions.push(Action::new(ActionType::Rest, "Rest".green()));
        }

        actions
    }

    fn handle_action(
        &self,
        _search: &str,
        action_type: ActionType,
        game_state: &mut GameState,
    ) -> Box<dyn Event> {
        match action_type {
            ActionType::Travel => Box::new(TravelEvent::new(game_state.get_locations())),
            _ => panic!("Unhandled action when handling action."),
        }
    }
}
