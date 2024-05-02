use crate::{
    actions::{Action, ActionType},
    characters::{player::Player, stats::Stats},
    game_state::GameState,
    item::Item,
    text_format::TextFormatter,
};

use super::{event_loop::EventLoop, event_response::EventResponse, visit_event::VisitEvent, Event};

pub struct ManageEvent {
    stats: Stats,
    inventory: Vec<Box<dyn Item>>,
}

impl ManageEvent {
    pub fn new(stats: Stats, inventory: Vec<Box<dyn Item>>) -> ManageEvent {
        ManageEvent { stats, inventory }
    }
}

impl Event for ManageEvent {
    fn prompt(&self) -> Option<String> {
        let stats = format!(
            "You have {} life, {} power and {} attack speed.",
            self.stats.life, self.stats.power, self.stats.attack_speed
        );

        let mut inventory = String::new();
        if self.inventory.is_empty() {
            inventory.push_str("You have no items in your inventory.")
        } else {
            inventory.push_str("You have these items in your inventory:");
            for item in &self.inventory {
                inventory = format!("{}\n{}", inventory, item.display_info());
            }
        }

        let response = format!("{}\n{}", stats, inventory);

        Some(response)
    }
    fn actions(&self) -> Vec<Action> {
        vec![Action::new(ActionType::Continue, "Continue".text_green())]
    }
    fn handle_action(
        &mut self,
        action: Action,
        game_state: &mut GameState,
        _player: &mut Player,
    ) -> EventResponse {
        match action.class {
            ActionType::Continue => {
                let next_event = VisitEvent::build(game_state);
                EventResponse::new(Some(next_event), None)
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}
