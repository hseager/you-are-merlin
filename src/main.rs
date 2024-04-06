use std::io;

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

        if let Some(response) = game.get_action_response(input.trim()) {
            println!("{response}");
        }

        game.handle_action(input.trim());
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
