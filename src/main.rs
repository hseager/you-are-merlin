use std::io;

use colored::Colorize;
use game_data::GameData;
use game_state::GameState;
use player_state::PlayerState;

use crate::theme::{get_theme, get_theme_display_list};

mod actions;
mod battle_manager;
mod characters;
mod config;
mod game_data;
mod game_state;
mod items;
mod player_state;
mod prompts;
mod theme;
mod utilities;

fn main() {
    let theme_selection = select_theme();
    let theme_data = get_theme(theme_selection);
    let game_data = GameData::new(theme_data);
    let mut game_state = GameState::new(&game_data);

    println!("You are {}.", &game_state.player.name);

    loop {
        let mut input = String::new();

        if let PlayerState::GameOver = game_state.state {
            println!("Game Over...");
            break;
        }

        if let PlayerState::Win = game_state.state {
            println!("You Win!");
            break;
        }

        game_state.get_prompt();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        game_state.handle_action(input.trim());
    }
}

fn select_theme() -> String {
    println!("{}", "Choose a theme.".bold());
    println!("{}", get_theme_display_list());

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to theme selection.");

    utilities::spacer();

    input.trim().to_string()
}

