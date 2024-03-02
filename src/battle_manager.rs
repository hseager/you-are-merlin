use std::{thread::sleep, time::Duration};

use crate::game_manager::{GameState, State};

pub fn handle_battle(game_state: &mut GameState) {
    let location = game_state.locations.get_mut(game_state.current_location);

    if let Some(location) = location {
        if let Some(encounter) = location.encounters.get_mut(location.current_encounter) {
            let player = &mut game_state.player;
            let enemy = &mut encounter.enemy;

            while enemy.life > 0 || player.life > 0 {
                enemy.life -= player.attack;

                println!(
                    "You attack {} for {} damage. (Enemy life: {})",
                    enemy.name, player.attack, enemy.life
                );

                if enemy.life <= 0 {
                    break;
                }

                sleep(Duration::from_secs(1));

                player.life -= enemy.attack;
                println!(
                    "{} attacks you for {} damage. (Your life: {})",
                    enemy.name, enemy.attack, player.life
                );

                if player.life <= 0 {
                    break;
                }

                sleep(Duration::from_secs(1));
            }

            if enemy.life <= 0 {
                println!("You defeated {}!", enemy.name);
                location.current_encounter += 1;
                enemy.life = 0;
            }

            if player.life <= 0 {
                println!("{} died!", player.name);
                game_state.state = State::GameOver;
            }
        } else {
            println!("No more encounters.");
        }
    } else {
        println!("Couldn't find location.");
    }
}
