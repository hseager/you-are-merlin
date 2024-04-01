use characters::Player;
use colored::ColoredString;
use config::{PLAYER_ATTACK, PLAYER_LIFE};
use game_data::GameData;
use game_state::GameState;
use player_state::PlayerState;
use theme::get_theme;

mod actions;
mod battle_manager;
mod characters;
mod config;
mod game_data;
mod game_state;
mod items;
mod player_state;
mod prompts;
pub mod theme;
pub mod utilities;

pub struct Game {
    game_state: GameState,
    player: Player,
}

impl Game {
    pub fn new(theme: String) -> Game {
        let theme_data = get_theme(theme);
        let game_data = GameData::new(theme_data);

        let player = Player {
            name: game_data.main_character.clone(),
            max_life: PLAYER_LIFE,
            life: PLAYER_LIFE,
            attack: PLAYER_ATTACK,
            inventory: Vec::new(),
        };

        let game_state = GameState::new(game_data);

        Game { game_state, player }
    }

    pub fn get_player_name(&self) -> &ColoredString {
        &self.player.name
    }

    pub fn is_running(&self) -> bool {
        !matches!(
            self.game_state.state,
            PlayerState::Win | PlayerState::GameOver
        )
    }

    pub fn get_prompt(&self) -> String {
        format!(
            "{}\n{}",
            self.game_state.get_prompt(),
            self.game_state.get_actions_display_list(&self.player)
        )
    }

    pub fn handle_action(&mut self, input: String) {
        self.game_state
            .handle_action(input.trim(), &mut self.player);
    }
}
