use colored::Colorize;

use crate::{
    actions::{Action, ActionType},
    game_data::entities::{Location, LocationType},
};

use super::event::Event;

pub struct VisitingEvent {
    current_location: Location,
    completed_locations: Vec<Location>,
}

impl VisitingEvent {
    pub fn new(location: Location, completed_locations: Vec<Location>) -> VisitingEvent {
        VisitingEvent {
            current_location: location,
            completed_locations,
        }
    }
}

impl Event for VisitingEvent {
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
}
