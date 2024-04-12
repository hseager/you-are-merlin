use characters::player::Player;
use colored::Colorize;
use event::{event_loop::event_loop_response::EventLoopResponse, Event};
use game_state::GameState;
use theme::theme_data::get_themes;
use utilities::map_text_color;
use wasm_bindgen::prelude::*;

use crate::{
    config::{PLAYER_ATTACK, PLAYER_LIFE},
    event::{event_response::EventResponse, visit_event::VisitEvent},
    game_data::GameData,
    theme::get_theme,
};

mod actions;
mod characters;
pub mod config;
mod event;
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

    pub fn is_running(&self) -> bool {
        self.game_state.is_running
    }

    pub fn get_prompt(&self) -> Option<String> {
        self.current_event.prompt()
    }

    pub fn get_actions(&self) -> Option<String> {
        let actions = self.current_event.actions();

        if !actions.is_empty() {
            Some(
                actions
                    .iter()
                    .map(|action| action.name.to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
            )
        } else {
            None
        }
    }

    pub fn handle_action(&mut self, search: &str) -> Option<String> {
        if let Some(action) = self.current_event.find_action(search) {
            let EventResponse {
                next_event,
                message,
            } = self.current_event.handle_action(
                search,
                action.class,
                &mut self.game_state,
                &mut self.player,
            );

            if let Some(event) = next_event {
                self.change_event(event);
            }

            message
        } else {
            Some(format!("This isn't the time to use {}!", search))
        }
    }

    fn change_event(&mut self, next_event: Box<dyn Event>) {
        self.current_event = next_event;
    }

    pub fn progress_event_loop(&mut self) -> String {
        let mut result = String::new();
        if let Some(event_loop) = self.current_event.get_event_loop() {
            let response = event_loop.progress_event_loop(&mut self.player, &mut self.game_state);

            match response {
                EventLoopResponse::InProgress(response_text) => result = response_text,
                EventLoopResponse::Complete(response_text, next_event) => {
                    self.change_event(next_event);
                    result = response_text;
                }
            }
        }
        result
    }

    pub fn has_event_loop(&mut self) -> bool {
        if let Some(event_loop) = self.current_event.get_event_loop() {
            event_loop.is_event_loop_active()
        } else {
            false
        }
    }

    pub fn get_event_loop_interval(&mut self) -> u8 {
        if let Some(event_loop) = self.current_event.get_event_loop() {
            event_loop.get_event_loop_interval()
        } else {
            0
        }
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
