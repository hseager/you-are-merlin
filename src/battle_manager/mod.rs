use rand::Rng;

use crate::{
    config::{ENEMY_BOSS_STATS, ENEMY_EASY_STATS, ENEMY_HARD_STATS, ENEMY_MEDIUM_STATS},
    game_data::entities::EnemyDifficulty,
};

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

pub fn map_theme_difficulty_to_stats(difficulty: EnemyDifficulty) -> (i16, u16) {
    match difficulty {
        EnemyDifficulty::Easy => ENEMY_EASY_STATS,
        EnemyDifficulty::Normal => ENEMY_MEDIUM_STATS,
        EnemyDifficulty::Hard => ENEMY_HARD_STATS,
        EnemyDifficulty::Boss => ENEMY_BOSS_STATS,
    }
}

// Select random damage from -2 to +2 of current attack stat
pub fn calculate_damage(attack: u16) -> u16 {
    let range = rand::thread_rng().gen_range(0..4);

    let mut damage = attack;

    if damage < 2 {
        damage = 2;
    }

    match range {
        0 => damage - 2,
        1 => damage - 1,
        2 => damage,
        3 => damage + 1,
        4 => damage + 2,
        _ => damage,
    }
}
