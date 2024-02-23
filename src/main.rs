use std::io;

mod actions;
mod encounter;
mod game_manager;
mod location;
mod player;
mod theme;
mod utilities;

use colored::Colorize;

use crate::game_manager::init_game;

fn main() {
    let mut game_state = init_game();

    println!("You are {}.", &game_state.player.name.bold());

    loop {
        let mut input = String::new();

        game_state.get_current_prompt();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        game_state.handle_action(&input.trim());
    }
}
