use crate::{
    actions::{Action, ActionType},
    characters::player::Player,
    game_data::entities::SideQuest,
    game_state::GameState,
    text_format::TextFormatter,
};

use super::{event_loop::EventLoop, event_response::EventResponse, visit_event::VisitEvent, Event};

pub struct SideQuestEvent {
    side_quest: SideQuest,
    accepted_quests: Vec<SideQuest>,
    has_quest_item_in_inventory: bool,
}

impl SideQuestEvent {
    pub fn new(
        quest: SideQuest,
        accepted_quests: Vec<SideQuest>,
        has_quest_item_in_inventory: bool,
    ) -> SideQuestEvent {
        SideQuestEvent {
            side_quest: quest,
            accepted_quests,
            has_quest_item_in_inventory,
        }
    }

    pub fn is_quest_accepted(&self) -> bool {
        self.accepted_quests
            .iter()
            .any(|q| q.character == self.side_quest.character)
    }
}

impl Event for SideQuestEvent {
    fn prompt(&self) -> Option<String> {
        if self.is_quest_accepted() {
            Some(format!(
                "You find a calm area. {} wants to ask you something.\n\
                \"Do you have it? Please, bring me {} back from {}.\"",
                self.side_quest.character, self.side_quest.item, self.side_quest.location_name
            ))
        } else {
            Some(format!(
                "You find a calm area. {} wants to ask you something.\n\
                \"Will you find {} from {} and bring it back to me? I will make it worth your while!\"",
                self.side_quest.character, self.side_quest.item, self.side_quest.location_name
            ))
        }
    }

    fn actions(&self) -> Vec<Action> {
        if self.has_quest_item_in_inventory {
            vec![Action::new(ActionType::GiveItem, "Give".text_blue())]
        } else if self.is_quest_accepted() {
            vec![Action::new(ActionType::Run, "Continue".text_green())]
        } else {
            vec![
                Action::new(ActionType::Accept, "Accept".text_green()),
                Action::new(ActionType::Run, "Decline".text_red()),
            ]
        }
    }

    fn handle_action(
        &mut self,
        action: Action,
        game_state: &mut GameState,
        player: &mut Player,
    ) -> EventResponse {
        match action.class {
            ActionType::Run => EventResponse::new(Some(VisitEvent::build(game_state)), None),
            ActionType::Accept => {
                game_state.accepted_quests.push(self.side_quest.clone());

                EventResponse::new(
                    Some(VisitEvent::build(game_state)),
                    Some("You accept their request.".to_string()),
                )
            }
            ActionType::GiveItem => {
                game_state
                    .completed_locations
                    .push(game_state.get_current_location().clone());

                let item = game_state.get_random_item();
                // player.equip_item(&item);

                let mut response_text = "\"Your assistance in retrieving this has been invaluable. Thank you for your help! Please take this.\"".to_string();

                response_text = format!(
                    "{}\nYou recieve {}! {}",
                    response_text,
                    item.name(),
                    item.display_stats()
                );

                EventResponse::new(Some(VisitEvent::build(game_state)), Some(response_text))
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}
