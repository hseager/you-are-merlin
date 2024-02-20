use colored::Color;

use crate::{
    actions::{Action, ActionItem}, event::{Event, EventItem}, location::get_location
};

pub struct GameState {
    pub current_event: EventItem,
}

pub fn init_game() -> GameState {
    let initial_actions = vec![
        ActionItem::new(Action::Explore, "Explore", Color::Yellow),
        ActionItem::new(Action::Travel, "Travel", Color::Blue),
    ];

    // TODO fix unwrap & clone
    let inital_event = EventItem {
        class: Event::Visiting,
        location: get_location("The White Mountains").unwrap().clone(),
        actions: initial_actions
    };

    GameState {
        current_event: inital_event
    }
}
