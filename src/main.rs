use std::io;

use colored::Colorize;
use theme::get_theme_display_list;
use you_are_merlin::{theme, utilities, Game};

fn main() {
    let theme = select_theme();
    let mut game = Game::new(theme);

    println!("{}", game.get_initial_prompt());

    while game.is_running() {
        let mut input = String::new();

        println!("{}", game.get_prompt());

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        game.handle_action(input);
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
