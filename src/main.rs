use colored::Colorize;
use std::io;

mod actions;
mod game_manager;
mod location;

use crate::game_manager::init_game;

fn main() {
    let game_state = init_game();
    let current_location = game_state.current_location.as_str().bold();

    println!("You are Merlin.");
    println!("You are currently at {current_location}.");
    println!("What would you like to do?");

    let options = game_state.get_actions_list();
    println!("Options: {options}");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim() == String::from("Travel") {
        println!("Where would you like to go?");
    } else {
        println!("Didn't get that...");
    }
}
