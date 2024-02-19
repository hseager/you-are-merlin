use colored::Color;

use crate::{
    actions::{Action, ActionItem},
    location::Location,
};

pub struct GameState {
    pub current_location: Location,
    pub actions: Vec<ActionItem>,
}

impl GameState {
    pub fn get_actions_list(&self) -> String {
        self.actions
            .iter()
            .map(|action| action.display_label())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn has_action(&self, search: &str) -> bool {
        self.actions
            .iter()
            .any(|action| action.label.to_lowercase() == search.to_lowercase())
    }

    // TODO better error handling
    pub fn find_action(&self, search: &str) -> &ActionItem {
        match self.actions
            .iter()
            .find(|action| action.label.to_lowercase() == search.to_lowercase()) {
                Some(action) => action,
                None => panic!("Couldn't find matched action")
            }
    }

    pub fn execute_action(&mut self, action: &Action) -> () {
        match action {
            Action::Explore => {
                self.actions = vec![
                    ActionItem::new(Action::Attack, "Attack", Color::Red),
                    ActionItem::new(Action::CastSpell, "Cast Spell", Color::Magenta)
                ];
                println!("You begin to explore {}, but a giant spider appears.", self.current_location.as_str());
            },
            Action::Travel => {
                println!("Where would you like to go?")
            },
            Action::Attack => {
                println!("You attack with your dagger. You did {} damage.", 3)
            },
            Action::CastSpell => {
                println!("Which spell do you cast?")
            }
        }
    }
}

pub fn init_game() -> GameState {
    let initial_actions = vec![
        ActionItem::new(Action::Explore, "Explore", Color::Yellow),
        ActionItem::new(Action::Travel, "Travel", Color::Blue),
    ];

    GameState {
        current_location: Location::DarklingWoods,
        actions: initial_actions,
    }
}
