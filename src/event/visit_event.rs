use colored::Colorize;

use crate::{
    actions::{Action, ActionType},
    characters::player::Player,
    game_data::entities::{Encounter, Location, LocationType, Quest},
    game_state::GameState,
};

use super::{
    battle_event::BattleEvent, event_loop::EventLoop, rest_event::RestEvent,
    side_quest_event::SideQuestEvent, travel_event::TravelEvent, Event, EventResponse,
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
    fn prompt(&self) -> Option<String> {
        Some(format!(
            "You are currently visiting {}. {}\nWhat would you like to do?",
            &self.current_location.name, &self.current_location.description
        ))
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
        player: &mut Player,
    ) -> EventResponse {
        match action_type {
            ActionType::Travel => {
                let next_event = Box::new(TravelEvent::new(game_state.get_locations()));
                EventResponse::new(Some(next_event), None)
            }
            ActionType::Rest => {
                let next_event = Box::new(RestEvent {});
                EventResponse::new(Some(next_event), None)
            }
            ActionType::Explore => match game_state.get_current_encounter() {
                Encounter::Battle(battle) => {
                    let next_event = Box::new(BattleEvent::new(battle.clone()));
                    EventResponse::new(Some(next_event), None)
                }
                Encounter::Quest(quest) => match quest {
                    Quest::SideQuest(quest) => {
                        let next_event = Box::new(SideQuestEvent::new(
                            quest.clone(),
                            game_state.accepted_quests.clone(),
                            player.has_item_in_inventory(&quest.item),
                        ));
                        EventResponse::new(Some(next_event), None)
                    }
                    Quest::MainQuest(_) => panic!("Not implemented MainQuest while visiting yet?"),
                },
                Encounter::BossFight(_) => panic!("Shouldn't be a boss fight when visiting?"),
            },
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}
