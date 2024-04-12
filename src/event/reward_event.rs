use colored::{ColoredString, Colorize};

use crate::{
    actions::{Action, ActionType},
    characters::player::Player,
    game_data::entities::{Location, SideQuest},
    game_state::GameState,
    items::create_item,
};

use super::{event_loop::EventLoop, event_response::EventResponse, visit_event::VisitEvent, Event};

pub struct RewardEvent {
    location: Location,
    quest_item: ColoredString,
}

impl RewardEvent {
    pub fn new(location: Location, quest_item: ColoredString) -> RewardEvent {
        RewardEvent {
            location,
            quest_item,
        }
    }

    pub fn is_on_side_quest(&self, accepted_quests: &[SideQuest], player: &Player) -> bool {
        accepted_quests.iter().any(|q| q.item == self.quest_item)
            && !player.has_item_in_inventory(&self.quest_item)
    }
}

impl Event for RewardEvent {
    fn prompt(&self) -> String {
        format!(
            "You successfully clear {} of dangers and stumble upon a safe area.\n\
            You spot a chest up ahead.",
            self.location.name
        )
    }

    fn actions(&self) -> Vec<Action> {
        vec![Action::new(ActionType::Run, "Open".yellow())]
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
                let mut response_text: String;

                let item = create_item(&mut game_state.items);
                player.equip_item(&item);

                response_text = format!(
                    "You find {}! Your attack power increases by {}, and your maximum life grows by {} points.",
                    item.name, item.attack, item.max_life
                );

                if self.is_on_side_quest(&game_state.accepted_quests, player) {
                    response_text =
                        format!("{}\nYou also find {}!", response_text, self.quest_item);
                    player.add_item_to_inventory(self.quest_item.clone());
                }

                // TODO Maybe do continue or travel event here
                let next_event = Box::new(VisitEvent::new(
                    game_state.get_current_location().clone(),
                    game_state.completed_locations.clone(),
                ));

                game_state.reset_encounters();

                Some(EventResponse::new(next_event, Some(response_text)))
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}
