use characters::Player;
use colored::Colorize;
use config::{PLAYER_ATTACK, PLAYER_LIFE};
use game_data::GameData;
use game_state::GameState;
use player_state::PlayerState;
use theme::{get_theme, theme_data::get_themes};
use wasm_bindgen::prelude::*;

use crate::utilities::map_text_color;

mod actions;
mod battle_manager;
mod characters;
pub mod config;
mod game_data;
mod game_state;
mod items;
mod player_state;
mod prompts;
pub mod theme;
pub mod utilities;

#[wasm_bindgen]
pub struct Game {
    game_state: GameState,
    player: Player,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(theme: String) -> Game {
        let theme_data = get_theme(theme);
        let game_data = GameData::new(theme_data);

        let player = Player {
            name: game_data.main_character.clone(),
            max_life: 150,
            life: PLAYER_LIFE,
            attack: PLAYER_ATTACK,
            inventory: Vec::new(),
        };

        let game_state = GameState::new(game_data);

        Game { game_state, player }
    }

    pub fn is_running(&self) -> bool {
        !matches!(
            self.game_state.state,
            PlayerState::Win | PlayerState::GameOver
        )
    }

    pub fn get_initial_prompt(&self) -> String {
        format!("You are {}.", &self.player.name.to_string())
    }

    pub fn get_prompt(&self) -> String {
        self.game_state.get_prompt()
    }

    pub fn get_actions_display_list(&self) -> String {
        self.game_state.get_actions_display_list(&self.player)
    }

    pub fn handle_action(&mut self, input: String) -> Option<String> {
        let response = self
            .game_state
            .handle_action(input.trim(), &mut self.player);

        self.game_state.actions = self.game_state.get_actions(&self.player);

        response
    }

    pub fn player_is_healing(&self) -> bool {
        matches!(self.game_state.state, PlayerState::Healing)
    }

    // TODO This is getting pretty messy having this here
    // Lets just see if it works in WASM and then refactor
    pub fn heal_player(&mut self) -> String {
        if self.player.life < self.player.max_life {
            self.player.heal()
        } else {
            self.game_state.state =
                PlayerState::Visiting(self.game_state.get_current_location().clone());

            self.game_state.actions = self.game_state.get_actions(&self.player);

            "You fully recover your health.".to_string()
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
