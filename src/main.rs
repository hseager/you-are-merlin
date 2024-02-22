use std::io;

// mod actions;
// mod event;
mod game_manager;
mod location;
mod player;
mod theme;
mod utilities;

use crate::game_manager::init_game;

fn main() {
    let game_state = init_game();

    println!("You are {}.", &game_state.player.name);

    loop {
        let mut input = String::new();

        game_state.get_current_prompt();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        game_state.handle_action(&input);

        // let current_event = &mut game_state.current_event;

        // match &current_event.class {
        //     Event::Travelling => {
        //         println!(
        //             "Where would you like to go? {}",
        //             get_locations_display_list()
        //         )
        //     }
        //     Event::Visiting => {
        //         println!(
        //             "You are currently at {}.",
        //             current_event.location.display_label()
        //         );
        //         println!("What would you like to do?");
        //         println!("Options: {}", current_event.get_actions_display_list());
        //     }
        // }

        // io::stdin()
        //     .read_line(&mut input)
        //     .expect("Failed to read line");

        // let input = input.trim();

        // match &current_event.class {
        //     Event::Travelling => match get_location(input) {
        //         Some(location) => current_event.location = location.clone(),
        //         None => println!(
        //             "That place is unknown, your map includes: {}",
        //             get_locations_display_list()
        //         ),
        //     },
        //     Event::Visiting => {
        //         if current_event.has_action(input) {
        //             current_event.execute_action(&input);
        //         } else {
        //             println!(
        //                 "This isn't the time to use that! Options: {}",
        //                 current_event.get_actions_display_list()
        //             );
        //         }
        //     }
        // }
    }
}
