use colored::Colorize;

use crate::{
    actions::{Action, ActionType},
    game_data::entities::{Encounter, Location, LocationType},
    game_state::GameState,
};

use super::{
    battle_event::BattleEvent, event_loop::EventLoop, travel_event::TravelEvent, Event,
    EventResponse,
};

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
        &mut self,
        _search: &str,
        action_type: ActionType,
        game_state: &mut GameState,
    ) -> Option<EventResponse> {
        match action_type {
            ActionType::Travel => {
                let next_event = Box::new(TravelEvent::new(game_state.get_locations()));
                Some(EventResponse::new(next_event))
            }
            ActionType::Explore => match game_state.get_current_encounter() {
                Encounter::Battle(battle) => {
                    let next_event = Box::new(BattleEvent::new(battle.clone()));
                    Some(EventResponse::new(next_event))
                }
                Encounter::BossFight(_) => None,
                Encounter::Quest(_) => None,
            },
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}
