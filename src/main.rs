use colored::Colorize;
use std::io;

mod actions;
mod game_manager;
mod location;

use crate::game_manager::init_game;

fn main() {
    let game_state = init_game();

    println!("You are Merlin.");
    println!("You are currently at {}.", game_state.current_location.as_str().bold());
    println!("What would you like to do?");
    println!("Options: {}", game_state.get_actions_list());

    loop {
        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        if game_state.has_action(input.trim()) {
            let action = game_state.find_action(input.trim());
            game_state.execute_action(&action.class);
        } else {
            println!("This isn't the time to use that! Options: {}", game_state.get_actions_list());
        }
    }
}
