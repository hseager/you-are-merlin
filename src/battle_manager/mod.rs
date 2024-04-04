use rand::Rng;

use crate::{
    config::{ENEMY_BOSS_STATS, ENEMY_EASY_STATS, ENEMY_HARD_STATS, ENEMY_MEDIUM_STATS},
    game_data::entities::EnemyDifficulty,
};

pub fn map_theme_enemy_difficulty_to_stats(difficulty: EnemyDifficulty) -> (i16, u16) {
    match difficulty {
        EnemyDifficulty::Easy => ENEMY_EASY_STATS,
        EnemyDifficulty::Normal => ENEMY_MEDIUM_STATS,
        EnemyDifficulty::Hard => ENEMY_HARD_STATS,
        EnemyDifficulty::Boss => ENEMY_BOSS_STATS,
    }
}

// Select random damage from -2 to +2 of current attack stat
pub fn calculate_damage(attack: u16) -> u16 {
    let damage_range = 2;
    rand::thread_rng().gen_range(attack - damage_range..=attack + damage_range)
}
