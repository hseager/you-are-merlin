use std::io;

use game_data::GameData;
use game_state::GameState;

mod actions;
mod game_state;
mod game_data;
mod characters;
mod config;
mod player_state;
mod theme;
mod utilities;

fn main() {
    let game_data = GameData::new();
    let mut game_state = GameState::new(&game_data);

    loop {
        let mut input = String::new();

        // TODO win/lose conditions

        game_state.get_prompt();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        game_state.handle_action(&input.trim(), &game_data);
    }
}

// use crate::{game_manager::Game, player_state::PlayerState};

// fn main() {
//     let mut game = Game::new();

//     println!("You are {}.", &game.player.name);

//     loop {
//         let mut input = String::new();

//         if let PlayerState::GameOver = game.state {
//             print!("Game Over...");
//             break;
//         }

//         if let PlayerState::Win = game.state {
//             print!("You Win!");
//             break;
//         }

//         game.get_current_prompt();

//         io::stdin()
//             .read_line(&mut input)
//             .expect("Failed to read line");

//         game.handle_action(&input.trim());
//     }
// }
