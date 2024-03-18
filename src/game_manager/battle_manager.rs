use std::{thread::sleep, time::Duration};

use crate::{
    characters::{Enemy, Fighter, Player},
    encounter::Encounter,
    game_manager::{Game, PlayerState},
};

enum BattleResult {
    Win(Enemy),
    Lose,
}

// TODO lose coupling of game

pub fn handle_battle(game: &mut Game) {
    let location = game.world.locations.get_mut(game.world.current_location);

    if let Some(location) = location {
        if let Some(encounter) = location.encounters.get_mut(location.current_encounter) {
            match encounter {
                Encounter::Battle(battle) => match start_battle(&mut game.player, &battle.enemy) {
                    BattleResult::Win(enemy) => {
                        println!("You defeated {}!", enemy.name);
                        location.go_to_next_encounter();
                    }
                    BattleResult::Lose => {
                        println!("{} died!", game.player.name);
                        game.state = PlayerState::GameOver;
                    }
                },
                Encounter::BossFight(battle) => {
                    match start_battle(&mut game.player, &battle.enemy) {
                        BattleResult::Win(enemy) => {
                            println!(
                                "You defeated {}! The {} is saved!",
                                enemy.name, game.world.name
                            );
                            game.state = PlayerState::Win;
                        }
                        BattleResult::Lose => {
                            println!("{} died!", game.player.name);
                            game.state = PlayerState::GameOver;
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

fn start_battle(player: &mut Player, enemy: &Enemy) -> BattleResult {
    // Don't directly mut enemy.life so that we can reset it after running from encounter and coming back
    let mut enemy_copy = enemy.clone();

    while enemy_copy.is_alive() || player.is_alive() {
        player.attack(&mut enemy_copy);

        if !enemy_copy.is_alive() {
            break;
        }

        sleep(Duration::from_secs(1));

        enemy_copy.attack(player);

        if !player.is_alive() {
            break;
        }

        sleep(Duration::from_secs(1));
    }

    if !enemy_copy.is_alive() {
        BattleResult::Win(enemy.clone())
    } else if !player.is_alive() {
        BattleResult::Lose
    } else {
        panic!("Battle ended without anyone dying?..");
    }
}
