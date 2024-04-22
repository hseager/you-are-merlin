use std::{io, thread::sleep, time::Duration};

use colored::{control, Colorize};
use you_are_merlin::{get_theme_display_list, utilities::spacer, Game};

fn main() {
    if cfg!(target_os = "windows") {
        control::set_virtual_terminal(true).unwrap();
    }

    let theme = select_theme();
    let mut game = Game::new(theme);

    println!("{}", game.get_intro());

    while game.is_running() {
        let mut input = String::new();

        if let Some(prompt) = game.get_prompt() {
            println!("{prompt}");
        }

        if let Some(actions) = game.get_actions() {
            println!("{}", actions);

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
        }

        if let Some(response) = game.handle_action(input.trim()) {
            println!("{response}");
        }

        while game.has_event_loop() {
            if let Some(response) = game.progress_event_loop() {
                println!("{}", response);
            }
            sleep(Duration::from_millis(game.get_event_loop_interval().into()));
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
