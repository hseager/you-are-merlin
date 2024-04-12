use colored::Colorize;

use crate::{
    actions::{Action, ActionType},
    characters::player::Player,
    game_data::entities::{Encounter, Quest, SideQuest},
    game_state::GameState,
    items::create_item,
};

use super::{event_loop::EventLoop, event_response::EventResponse, visit_event::VisitEvent, Event};

pub struct SideQuestEvent {
    side_quest: SideQuest,
    accepted_quests: Vec<SideQuest>,
    has_quest_item_in_invetory: bool,
}

impl SideQuestEvent {
    pub fn new(
        quest: SideQuest,
        accepted_quests: Vec<SideQuest>,
        has_quest_item_in_invetory: bool,
    ) -> SideQuestEvent {
        SideQuestEvent {
            side_quest: quest,
            accepted_quests,
            has_quest_item_in_invetory,
        }
    }

    pub fn is_quest_accepted(&self) -> bool {
        self.accepted_quests
            .iter()
            .any(|q| q.character == self.side_quest.character)
    }
}

impl Event for SideQuestEvent {
    fn prompt(&self) -> String {
        if self.is_quest_accepted() {
            format!(
                "You find a calm area. {} wants to ask you something.\n\
                \"Do you have it? Please, bring me {} back from {}.\"",
                self.side_quest.character, self.side_quest.item, self.side_quest.location_name
            )
        } else {
            format!(
                "You find a calm area. {} wants to ask you something.\n\
                \"Will you find {} from {} and bring it back to me? I will make it worth your while!\"",
                self.side_quest.character, self.side_quest.item, self.side_quest.location_name
            )
        }
    }

    fn actions(&self) -> Vec<Action> {
        if self.has_quest_item_in_invetory {
            vec![Action::new(ActionType::GiveItem, "Give".blue())]
        } else if self.is_quest_accepted() {
            vec![Action::new(ActionType::Run, "Continue".green())]
        } else {
            vec![
                Action::new(ActionType::Accept, "Accept".green()),
                Action::new(ActionType::Run, "Decline".red()),
            ]
        }
    }

    fn handle_action(
        &mut self,
        _search: &str,
        action_type: ActionType,
        game_state: &mut GameState,
        player: &mut Player,
    ) -> Option<EventResponse> {
        match action_type {
            ActionType::Run => {
                let next_event = Box::new(VisitEvent::new(
                    game_state.get_current_location().clone(),
                    game_state.completed_locations.clone(),
                ));
                Some(EventResponse::new(next_event, None))
            }
            ActionType::Accept => {
                self.accepted_quests.push(self.side_quest.clone());

                let next_event = Box::new(VisitEvent::new(
                    game_state.get_current_location().clone(),
                    game_state.completed_locations.clone(),
                ));

                Some(EventResponse::new(
                    next_event,
                    Some("You accept their request.".to_string()),
                ))
            }
            ActionType::GiveItem => {
                game_state
                    .completed_locations
                    .push(game_state.get_current_location().clone());

                let item = create_item(&mut game_state.items);
                player.equip_item(&item);

                let mut response_text = "\"Your assistance in retrieving this has been invaluable. Thank you for your help! Please take this.\"".to_string();

                response_text = format!(
                    "{}\nYou recieve {}! Your attack power increases by {}, and your maximum life grows by {} points.",
                    response_text, item.name, item.attack, item.max_life
                );

                let next_event = Box::new(VisitEvent::new(
                    game_state.get_current_location().clone(),
                    game_state.completed_locations.clone(),
                ));
                Some(EventResponse::new(next_event, Some(response_text)))
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}
