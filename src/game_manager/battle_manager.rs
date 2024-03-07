use std::{thread::sleep, time::Duration};

use rand::Rng;

use crate::{
    encounter::Encounter,
    enemy::Enemy,
    game_manager::{GameState, State},
    player::Player,
};

enum BattleResult {
    Win(Enemy),
    Lose,
}

pub fn handle_battle(game_state: &mut GameState) {
    let location = game_state.locations.get_mut(game_state.current_location);

    if let Some(location) = location {
        if let Some(encounter) = location.encounters.get_mut(location.current_encounter) {
            match encounter {
                Encounter::Battle(battle) => {
                    match start_battle(&mut game_state.player, &mut battle.enemy) {
                        BattleResult::Win(enemy) => {
                            println!("You defeated {}!", enemy.name);
                            location.current_encounter += 1;
                        }
                        BattleResult::Lose => {
                            println!("{} died!", game_state.player.name);
                            game_state.state = State::GameOver;
                        }
                    }
                }
                _ => (),
            }
        } else {
            println!("No more encounters.");
        }
    } else {
        println!("Couldn't find location.");
    }
}

fn start_battle(player: &mut Player, enemy: &mut Enemy) -> BattleResult {
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

    // TODO don't mut enemy.life so we can reset after running
    if enemy.life <= 0 {
        enemy.life = 0;
        return BattleResult::Win(*enemy);
    }

    if player.life <= 0 {
        BattleResult::Lose
    } else {
        BattleResult::Win(*enemy)
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
