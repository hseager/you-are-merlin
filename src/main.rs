use std::io;

mod actions;
mod characters;
mod config;
mod encounter;
mod game_manager;
mod location;
mod player_state;
mod theme;
mod utilities;
mod world;

use crate::{game_manager::init_game, player_state::PlayerState};

fn main() {
    let mut game_state = init_game();

    println!("You are {}.", &game_state.player.name);

    loop {
        let mut input = String::new();

        if let PlayerState::GameOver = game_state.state {
            print!("Game Over...");
            break;
        }

        if let PlayerState::Win = game_state.state {
            print!("You Win!");
            break;
        }

        game_state.get_current_prompt();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        game_state.handle_action(&input.trim());
    }
}
