use crate::{
    actions::{Action, ActionType},
    characters::player::Player,
    game_data::entities::{Location, SideQuest},
    game_state::GameState,
    item::quest_item::QuestItem,
    text_format::TextFormatter,
};

use super::{
    event_loop::EventLoop, event_response::EventResponse, travel_event::TravelEvent, Event,
};

use crate::item::Item;

enum RewardState {
    Discover,
    Recieved,
}

pub struct RewardEvent {
    location: Location,
    quest_item: QuestItem,
    state: RewardState,
}

impl RewardEvent {
    pub fn new(location: Location, quest_item: QuestItem) -> RewardEvent {
        RewardEvent {
            location,
            quest_item,
            state: RewardState::Discover,
        }
    }

    pub fn is_on_side_quest(&self, accepted_quests: &[SideQuest], player: &Player) -> bool {
        accepted_quests
            .iter()
            .any(|q| q.item.name() == self.quest_item.name())
            && !player.has_item_in_inventory(Box::new(self.quest_item.clone()))
    }
}

impl Event for RewardEvent {
    fn prompt(&self) -> Option<String> {
        if let RewardState::Discover = self.state {
            Some(format!(
                "You successfully clear {} of dangers and stumble upon a safe area.\n\
                You spot a chest up ahead.",
                self.location.name
            ))
        } else {
            None
        }
    }

    fn actions(&self) -> Vec<Action> {
        if let RewardState::Discover = self.state {
            vec![Action::new(ActionType::Open, "Open".text_yellow())]
        } else {
            vec![Action::new(ActionType::Continue, "Continue".text_green())]
        }
    }

    fn handle_action(
        &mut self,
        action: Action,
        game_state: &mut GameState,
        player: &mut Player,
    ) -> EventResponse {
        match action.class {
            ActionType::Open => {
                let mut response_text: String;

                let item = game_state.get_random_item();
                // player.equip_item(&item);

                response_text = format!("You find:{}", item.display_info());

                if self.is_on_side_quest(&game_state.accepted_quests, player) {
                    response_text = format!(
                        "{}\n\
                        You also find: {}",
                        response_text,
                        self.quest_item.display_info()
                    );
                    player.add_item_to_inventory(Box::new(self.quest_item.clone()));
                }

                player.add_item_to_inventory(item);

                game_state.reset_encounters();

                self.state = RewardState::Recieved;

                EventResponse::new(None, Some(response_text))
            }
            ActionType::Continue => {
                let next_event = Box::new(TravelEvent::new(game_state.get_locations()));

                EventResponse::new(Some(next_event), None)
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}
