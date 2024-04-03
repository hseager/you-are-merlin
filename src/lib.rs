use characters::player::Player;
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

#[wasm_bindgen]
#[allow(dead_code)]
pub struct Game {
    pub config: WasmConfig,
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
            config,
            game_state,
            player,
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

    // let below = techDebt + 1 yuck
    pub fn player_is_fighting(&self) -> bool {
        matches!(self.game_state.state, PlayerState::Fighting)
    }

    pub fn player_attack_enemy(&mut self) -> String {
        match self.game_state.get_current_encounter() {
            Encounter::Battle(battle) => {
                let mut enemy = battle.enemy.clone();

                if enemy.is_alive() {
                    self.player.attack(&mut enemy)
                } else {
                    self.game_state.go_to_next_encounter(&mut self.player);
                    format!("You defeated {}!", enemy.name)
                }
            }
            Encounter::BossFight(battle) => {
                let mut enemy = battle.enemy.clone();

                if enemy.is_alive() {
                    self.player.attack(&mut enemy)
                } else {
                    self.game_state.state = PlayerState::Win;
                    format!(
                        "You defeated {}! {} is saved!",
                        enemy.name, &self.game_state.game_data.world_name
                    )
                }
            }
            _ => panic!("Shouldn't be fighting when not in a battle"),
        }

        // match game_state.get_current_encounter() {
        //     Encounter::Battle(battle) => match start_battle(player, &battle.enemy) {
        //         BattleResult::Win(enemy) => {
        //             println!("You defeated {}!", enemy.name);
        //             game_state.go_to_next_encounter(player);
        //         }
        //         BattleResult::Lose => {
        //             println!("{} died!", &player.name);
        //             game_state.state = PlayerState::GameOver;
        //         }
        //     },
        //     Encounter::BossFight(battle) => match start_battle(player, &battle.enemy) {
        //         BattleResult::Win(enemy) => {
        //             println!(
        //                 "You defeated {}! {} is saved!",
        //                 enemy.name, game_state.game_data.world_name
        //             );
        //             game_state.state = PlayerState::Win;
        //         }
        //         BattleResult::Lose => {
        //             println!("{} died!", player.name);
        //             game_state.state = PlayerState::GameOver;
        //         }
        //     },
        //     _ => (),
        // }
    }

    pub fn enemy_attack_player(&mut self) -> String {
        match self.game_state.get_current_encounter() {
            Encounter::Battle(battle) | Encounter::BossFight(battle) => {
                let enemy = battle.enemy.clone();

                if self.player.is_alive() {
                    enemy.attack(&mut self.player)
                } else {
                    self.game_state.state = PlayerState::GameOver;
                    format!("{} died!", self.player.name)
                }
            }
            _ => panic!("Shouldn't be fighting when not in a battle"),
        }
    }

    pub fn is_enemy_alive(&self) -> bool {
        match self.game_state.get_current_encounter() {
            Encounter::Battle(battle) | Encounter::BossFight(battle) => {
                let enemy = battle.enemy.clone();

                enemy.is_alive()
            }
            _ => panic!("Shouldn't be an enemy when not in a battle"),
        }
    }

    pub fn is_player_alive(&self) -> bool {
        self.player.is_alive()
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

#[wasm_bindgen]
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct WasmConfig {
    pub rest_interval_seconds: usize,
    pub battle_interval_seconds: u8,
}
