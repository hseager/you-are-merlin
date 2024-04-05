use std::{io, thread::sleep, time::Duration};

use colored::Colorize;
use events::visiting_event::VisitingEvent;
use you_are_merlin::{
    config::{BATTLE_INTERVAL_SECONDS, REST_INTERVAL_SECONDS},
    get_theme_display_list,
    utilities::spacer,
    Game,
};

mod actions;
mod events;
mod game_data;

fn main() {
    let visiting_event = VisitingEvent::new();
}

fn main1() {
    let theme = select_theme();
    let mut game = Game::new(theme);

    println!("{}", game.get_initial_prompt());

    while game.is_running() {
        let mut input = String::new();

        println!("{}", game.get_prompt());
        println!("{}", game.get_actions_display_list());

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Some(response) = game.handle_action(input) {
            println!("{response}");
        }

        // This logic shouldn't live here, but need to find a way to handle loops & sleep in both rust CLI and WASM...
        while game.player_is_healing() {
            println!("{}", game.heal_player());
            sleep(Duration::from_secs(REST_INTERVAL_SECONDS as u64));
        }

        // This logic shouldn't live here, but need to find a way to handle loops & sleep in both rust CLI and WASM...
        while game.player_is_fighting() {
            println!("{}", game.handle_battle());
            sleep(Duration::from_secs(BATTLE_INTERVAL_SECONDS as u64));
        }
    }
}

fn select_theme() -> String {
    println!("{}", "Choose a theme.".bold());
    println!("{}", get_theme_display_list());

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to theme selection.");

    spacer();

    input.trim().to_string()
}
