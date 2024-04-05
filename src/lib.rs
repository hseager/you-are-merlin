use characters::player::Player;
use colored::Colorize;
use events::event::Event;
use game_state::GameState;
use theme::theme_data::get_themes;
use utilities::map_text_color;
use wasm_bindgen::prelude::*;

use crate::{
    config::{PLAYER_ATTACK, PLAYER_LIFE},
    events::visit_event::VisitEvent,
    game_data::GameData,
    theme::get_theme,
};

mod actions;
mod battle_manager;
mod characters;
pub mod config;
mod events;
mod game_data;
mod game_state;
mod items;
pub mod theme;
pub mod utilities;

#[wasm_bindgen]
#[allow(dead_code)]
pub struct Game {
    player: Player,
    game_state: GameState,
    current_event: Box<dyn Event>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(theme: String) -> Game {
        let theme_data = get_theme(theme);
        let game_data = GameData::new(theme_data);
        let player = Player::new(game_data.main_character.clone(), PLAYER_LIFE, PLAYER_ATTACK);
        let location = game_data
            .locations
            .first()
            .expect("Unable to get location when creating a new game.");

        let first_event = VisitEvent::new(location.clone(), Vec::new());

        let game_state = GameState::new(game_data);

        Game {
            player,
            game_state,
            current_event: Box::new(first_event),
        }
    }

    pub fn get_intro(&self) -> String {
        format!("You are {}.", &self.player.name.to_string())
    }

    // TODO check game status
    pub fn is_running(&self) -> bool {
        true
    }

    pub fn get_prompt(&self) -> String {
        self.current_event.prompt()
    }

    pub fn get_actions(&self) -> String {
        self.current_event
            .actions()
            .iter()
            .map(|action| action.name.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn handle_action(&mut self, search: &str) -> Option<String> {
        match self.current_event.find_action(search) {
            Some(action) => {
                let next_event =
                    self.current_event
                        .handle_action(search, action.class, &mut self.game_state);

                self.change_event(next_event);
                None
            }
            None => Some(format!("This isn't the time to use {}!", search)),
        }
    }

    fn change_event(&mut self, next_event: Box<dyn Event>) {
        self.current_event = next_event;
    }
}

#[wasm_bindgen]
pub fn get_theme_display_list() -> String {
    let themes = get_themes();

    let mut joined_themes = String::new();
    for (i, theme) in themes.iter().enumerate() {
        joined_themes.push_str(&theme.0.color(map_text_color(i)).to_string());
        if i != themes.len() - 1 {
            joined_themes.push_str(", ");
        }
    }
    joined_themes
}
