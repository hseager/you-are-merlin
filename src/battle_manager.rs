use std::{thread::sleep, time::Duration};

use crate::{
    characters::{Enemy, Fighter, Player},
    config::BATTLE_INTERVAL_SECONDS,
    game_data::entities::Encounter,
    game_state::GameState,
    player_state::PlayerState,
};

enum BattleResult {
    Win(Enemy),
    Lose,
}

pub fn handle_battle(game_state: &mut GameState) {
    match game_state.get_current_encounter() {
        Encounter::Battle(battle) => match start_battle(&mut game_state.player, &battle.enemy) {
            BattleResult::Win(enemy) => {
                println!("You defeated {}!", enemy.name);
                game_state.go_to_next_encounter();
            }
            BattleResult::Lose => {
                println!("{} died!", &game_state.player.name);
                game_state.state = PlayerState::GameOver;
            }
        },
        Encounter::BossFight(battle) => match start_battle(&mut game_state.player, &battle.enemy) {
            BattleResult::Win(enemy) => {
                println!(
                    "You defeated {}! The {} is saved!",
                    enemy.name, game_state.game_data.world_name
                );
                game_state.state = PlayerState::Win;
            }
            BattleResult::Lose => {
                println!("{} died!", game_state.player.name);
                game_state.state = PlayerState::GameOver;
            }
        },
        _ => (),
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

        sleep(Duration::from_secs(BATTLE_INTERVAL_SECONDS as u64));

        enemy_copy.attack(player);

        if !player.is_alive() {
            break;
        }

        sleep(Duration::from_secs(BATTLE_INTERVAL_SECONDS as u64));
    }

    if !enemy_copy.is_alive() {
        BattleResult::Win(enemy.clone())
    } else if !player.is_alive() {
        BattleResult::Lose
    } else {
        panic!("Battle ended without anyone dying?..");
    }
}
