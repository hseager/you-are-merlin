use std::{thread::sleep, time::Duration};

use rand::Rng;

use crate::game_manager::{GameState, State};

pub fn handle_battle(game_state: &mut GameState) {
    let location = game_state.locations.get_mut(game_state.current_location);

    if let Some(location) = location {
        if let Some(encounter) = location.encounters.get_mut(location.current_encounter) {
            let player = &mut game_state.player;
            let enemy = &mut encounter.enemy;

            while enemy.life > 0 || player.life > 0 {
                let player_damage = calculate_damage(player.attack);

                enemy.life -= player_damage;

                println!(
                    "You attack {} for {} damage. (Enemy life: {})",
                    enemy.name, player_damage, enemy.life
                );

                if enemy.life <= 0 {
                    break;
                }

                sleep(Duration::from_secs(1));

                let enemy_damage = calculate_damage(enemy.attack);
                player.life -= enemy_damage;
                println!(
                    "{} attacks you for {} damage. (Your life: {})",
                    enemy.name, enemy_damage, player.life
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

fn calculate_damage(damage: i16) -> i16 {
    let range = rand::thread_rng().gen_range(0..4);

    match range {
        0 => damage - 2,
        1 => damage - 1,
        2 => damage,
        3 => damage + 1,
        4 => damage + 2,
        _ => damage,
    }
}
