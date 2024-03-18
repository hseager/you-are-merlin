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

use crate::{game_manager::Game, player_state::PlayerState};

fn main() {
    let mut game = Game::new();

    println!("You are {}.", &game.player.name);

    loop {
        let mut input = String::new();

        if let PlayerState::GameOver = game.state {
            print!("Game Over...");
            break;
        }

        if let PlayerState::Win = game.state {
            print!("You Win!");
            break;
        }

        game.get_current_prompt();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        game.handle_action(&input.trim());
    }
}
