use characters::player::Player;
use event::{event_loop::event_loop_response::EventLoopResponse, Event};
use game_state::GameState;
use text_format::TextFormatter;
use theme::theme_data::get_themes;
use wasm_bindgen::prelude::*;

use crate::{
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
pub mod item;
pub mod text_format;
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
        let player = Player::new(game_data.main_character.clone());
        let mut game_state = GameState::new(game_data);
        let current_event = VisitEvent::build(&mut game_state);

        Game {
            player,
            game_state,
            current_event,
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
            } = self
                .current_event
                .handle_action(action, &mut self.game_state, &mut self.player);

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

    pub fn progress_event_loop(&mut self, current_epoch_milli: i32) -> Option<String> {
        let mut result = None;
        if let Some(event_loop) = self.current_event.get_event_loop() {
            let response = event_loop.progress_event_loop(
                current_epoch_milli,
                &mut self.player,
                &mut self.game_state,
            );

            match response {
                EventLoopResponse::InProgress(response_text) => result = response_text,
                EventLoopResponse::Complete(response_text, next_event) => {
                    self.change_event(next_event);
                    result = Some(response_text);
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

    pub fn get_event_loop_interval(&mut self) -> u32 {
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
        joined_themes.push_str(&theme.0.text_color(i).to_string());
        if i != themes.len() - 1 {
            joined_themes.push_str(", ");
        }
    }
    joined_themes
}
