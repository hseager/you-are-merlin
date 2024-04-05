use characters::{enemy::Enemy, player::Player};
use colored::Colorize;
use config::{PLAYER_ATTACK, PLAYER_LIFE};
use game_data::GameData;
use game_state::GameState;
use player_state::PlayerState;
use theme::{get_theme, theme_data::get_themes};
use wasm_bindgen::prelude::*;

use crate::{
    characters::fighter::Fighter,
    config::{BATTLE_INTERVAL_SECONDS, REST_INTERVAL_SECONDS},
    game_data::entities::Encounter,
    utilities::map_text_color,
};

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

mod events;

enum Turn {
    Player,
    Enemy,
}

#[wasm_bindgen]
#[allow(dead_code)]
pub struct Game {
    game_state: GameState,
    player: Player,
    current_target: Option<Enemy>,
    attack_turn: Option<Turn>,
    pub config: WasmConfig,
}

#[wasm_bindgen]
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct WasmConfig {
    pub rest_interval_seconds: usize,
    pub battle_interval_seconds: u8,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
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

        let config = WasmConfig {
            rest_interval_seconds: REST_INTERVAL_SECONDS,
            battle_interval_seconds: BATTLE_INTERVAL_SECONDS,
        };

        Game {
            game_state,
            player,
            current_target: None,
            attack_turn: None,
            config,
        }
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
        self.game_state.actions = self.game_state.get_actions(&self.player);

        let response = self
            .game_state
            .handle_action(input.trim(), &mut self.player);

        response
    }

    pub fn player_is_healing(&self) -> bool {
        matches!(self.game_state.state, PlayerState::Healing)
    }

    // TODO This is getting pretty messy having this here
    // Lets just see if it works in WASM and then refactor later
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

    pub fn player_is_fighting(&self) -> bool {
        matches!(self.game_state.state, PlayerState::Fighting)
    }

    // TODO What a mess, huge refactor needed...
    pub fn handle_battle(&mut self) -> String {
        // This gets run in a loop so we need to store the enemy somewhere so we can
        // track it's life each iteration
        if self.current_target.is_none() {
            self.current_target = Some(self.game_state.get_current_enemy());
        }

        if self.attack_turn.is_none() {
            self.attack_turn = Some(Turn::Player)
        }

        let is_boss_fight = matches!(
            self.game_state.get_current_encounter(),
            Encounter::BossFight(_)
        );

        let enemy = self
            .current_target
            .as_mut()
            .expect("Should always be a current target in a battle.");

        let mut result = String::new();

        match self.attack_turn {
            Some(Turn::Player) => {
                result = self.player.attack(enemy);

                if !enemy.is_alive() {
                    if is_boss_fight {
                        result = format!(
                            "You defeated {}! {} is saved!\nYou win!",
                            enemy.name, self.game_state.game_data.world_name
                        );
                        self.game_state.state = PlayerState::Win;
                    } else {
                        result = format!("You defeated {}!", enemy.name);
                        if let Some(reward_text) =
                            self.game_state.go_to_next_encounter(&mut self.player)
                        {
                            result = format!("{}\n{}", result, reward_text);
                        }
                    }

                    self.current_target = None;
                    self.attack_turn = None;
                } else {
                    self.attack_turn = Some(Turn::Enemy);
                }
            }
            Some(Turn::Enemy) => {
                result = enemy.attack(&mut self.player);

                if !self.player.is_alive() {
                    self.game_state.state = PlayerState::GameOver;
                    result = format!("{} died!\nGame Over...", self.player.name);
                    self.current_target = None;
                    self.attack_turn = None;
                } else {
                    self.attack_turn = Some(Turn::Player);
                }
            }
            None => (),
        }

        result
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
