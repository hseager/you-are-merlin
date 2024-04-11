use std::{io, thread::sleep, time::Duration};

use colored::Colorize;
use you_are_merlin::{get_theme_display_list, utilities::spacer, Game};

fn main() {
    let theme = select_theme();
    let mut game = Game::new(theme);

    println!("{}", game.get_intro());

    while game.is_running() {
        let mut input = String::new();

        println!("{}", game.get_prompt());
        println!("{}", game.get_actions());

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if let Some(response) = game.handle_action(input.trim()) {
            println!("{response}");
        }

        while game.has_event_loop() {
            println!("{}", game.progress_event_loop());
            sleep(Duration::from_secs(game.get_event_loop_interval()));
        }
    }
}

fn select_theme() -> String {
    println!("{}", "Choose a theme.".bold());
    println!("{}", get_theme_display_list());

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to get theme selection.");

    spacer();

    input.trim().to_string()
}
