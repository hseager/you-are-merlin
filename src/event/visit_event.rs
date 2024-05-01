use crate::{
    actions::{Action, ActionType},
    characters::player::Player,
    game_data::entities::{Encounter, Location, LocationType, Quest},
    game_state::GameState,
    text_format::TextFormatter,
};

use super::{
    battle_event::BattleEvent, event_loop::EventLoop, main_quest_event::MainQuestEvent,
    manage_event::ManageEvent, rest_event::RestEvent, side_quest_event::SideQuestEvent,
    travel_event::TravelEvent, Event, EventResponse,
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

    pub fn build(game_state: &mut GameState) -> Box<dyn Event> {
        Box::new(VisitEvent::new(
            game_state.get_current_location().clone(),
            game_state.completed_locations.clone(),
        ))
    }
}

impl Event for VisitEvent {
    fn prompt(&self) -> Option<String> {
        Some(format!(
            "You are currently visiting {}.\n\
            {}\n\
            What would you like to do?",
            &self.current_location.name, &self.current_location.description
        ))
    }

    fn actions(&self) -> Vec<Action> {
        let mut actions = vec![];

        if !self
            .completed_locations
            .iter()
            .any(|l| l.name == self.current_location.name)
        {
            actions.push(Action::new(ActionType::Explore, "Explore".text_blue()))
        }

        if let LocationType::SafeZone = &self.current_location.class {
            actions.push(Action::new(ActionType::Rest, "Rest".text_green()));
        }

        actions.push(Action::new(ActionType::Manage, "Manage".text_cyan()));
        actions.push(Action::new(ActionType::Travel, "Travel".text_yellow()));

        actions
    }

    fn handle_action(
        &mut self,
        action: Action,
        game_state: &mut GameState,
        player: &mut Player,
    ) -> EventResponse {
        match action.class {
            ActionType::Travel => {
                let next_event = Box::new(TravelEvent::new(game_state.get_locations()));
                EventResponse::new(Some(next_event), None)
            }
            ActionType::Rest => {
                let next_event = Box::new(RestEvent::new());
                EventResponse::new(Some(next_event), None)
            }
            ActionType::Manage => {
                let next_event = Box::new(ManageEvent::new(
                    player.stats.clone(),
                    player.inventory.clone(),
                ));
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
                            player.has_item_in_inventory(Box::new(quest.item.clone())),
                        ));
                        EventResponse::new(Some(next_event), None)
                    }
                    Quest::MainQuest(quest) => {
                        let next_event = Box::new(MainQuestEvent::new(quest.clone()));
                        EventResponse::new(Some(next_event), None)
                    }
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
