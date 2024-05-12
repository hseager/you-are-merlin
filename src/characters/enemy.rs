use std::{iter::Take, slice::Iter};

use crate::{
    characters::fighter::{calculate_damage, handle_block, is_critical, is_dodge, is_parry},
    config::{
        ENEMY_BOSS_STATS, ENEMY_BOSS_STAT_COUNT, ENEMY_EASY_STATS, ENEMY_EASY_STAT_COUNT,
        ENEMY_HARD_STATS, ENEMY_HARD_STAT_COUNT, ENEMY_MEDIUM_STATS, ENEMY_MEDIUM_STAT_COUNT,
        ENEMY_STAT_ATTACK_SPEED_RANGE, ENEMY_STAT_BLOCK_RANGE, ENEMY_STAT_CRIT_CHANCE_RANGE,
        ENEMY_STAT_CRIT_MULTI_RANGE, ENEMY_STAT_DODGE_CHANCE, ENEMY_STAT_MAX_LIFE_RANGE,
        ENEMY_STAT_PARRY_CHANCE, ENEMY_STAT_POWER_RANGE, FIGHTER_BASE_ATTACK_SPEED,
    },
    item::Stat,
};
use rand::{prelude::SliceRandom, thread_rng, Rng};

use super::{fighter::Fighter, stats::Stats};
use crate::text_format::TextFormatter;

#[derive(Clone, Debug)]
pub struct Enemy {
    pub name: String,
    pub description: &'static str,
    pub difficulty: EnemyDifficulty,
    pub stats: Stats,
}

impl Enemy {
    pub fn new(
        name: String,
        description: &'static str,
        difficulty: EnemyDifficulty,
        stats: Stats,
    ) -> Enemy {
        Enemy {
            name,
            description,
            difficulty,
            stats,
        }
    }
}

impl Fighter for Enemy {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn life(&self) -> i16 {
        self.stats.life
    }

    fn attack(&mut self, target: &mut dyn Fighter) -> String {
        let mut damage = calculate_damage(self.stats.power);
        let is_crit = is_critical(self.crit_chance());
        let mut action_message = String::from("attacks");

        if is_crit {
            action_message = "CRITS".text_red_bold();
            damage = (damage as f32 * self.crit_multiplier()).round() as u16;
        }

        // Handle dodge
        let mut status_text = String::new();
        let is_dodge = is_dodge(target.dodge());
        if is_dodge {
            status_text = String::from("But you dodged! ");
        }

        // Handle parry
        let mut parry_damage = 0;
        let is_parry: bool = is_parry(target.parry());
        if is_parry {
            parry_damage = damage / 2;
            self.take_damage(parry_damage);
        }

        // Handle block
        let mut blocked_damage = 0;
        if !is_dodge {
            blocked_damage = handle_block(damage, target.block());

            damage = damage.saturating_sub(parry_damage);
            damage = damage.saturating_sub(blocked_damage);

            target.take_damage(damage);
        }

        if !is_dodge {
            if is_parry && blocked_damage > 0 {
                status_text = format!(
                    "You parry and reflect {} damage and block {} damage. ",
                    parry_damage.to_string().text_bold(),
                    blocked_damage.to_string().text_bold()
                );
            } else if is_parry && blocked_damage == 0 {
                status_text = format!(
                    "You parry and reflect {} damage. ",
                    parry_damage.to_string().text_bold(),
                );
            } else if blocked_damage > 0 {
                status_text = format!(
                    "You block {} damage. ",
                    blocked_damage.to_string().text_bold()
                );
            }
        }

        format!(
            "{} {} you for {} damage. {}(Your life: {})",
            &self.name(),
            action_message,
            damage.to_string().text_bold(),
            status_text,
            &target.life().to_string().text_red()
        )
    }

    fn take_damage(&mut self, damage: u16) {
        self.stats.life -= damage as i16;
    }

    fn attack_speed_as_milliseconds(&self) -> u16 {
        FIGHTER_BASE_ATTACK_SPEED - (self.attack_speed() * 10)
    }

    fn power(&self) -> u16 {
        self.stats.power
    }
    fn attack_speed(&self) -> u16 {
        self.stats.attack_speed
    }
    fn max_life(&self) -> i16 {
        self.stats.max_life
    }
    fn block(&self) -> u16 {
        self.stats.block
    }
    fn crit_multiplier(&self) -> f32 {
        self.stats.crit_multiplier
    }
    fn crit_chance(&self) -> f32 {
        self.stats.crit_chance
    }
    fn parry(&self) -> f32 {
        self.stats.parry
    }
    fn dodge(&self) -> f32 {
        self.stats.dodge
    }
}

#[derive(Clone, Copy, Debug)]
pub enum EnemyDifficulty {
    Easy,
    Normal,
    Hard,
    Boss,
}

impl EnemyDifficulty {
    pub fn description(&self) -> String {
        match self {
            EnemyDifficulty::Easy => {
                String::from("They pose little threat and should be dispatched with ease.")
            }
            EnemyDifficulty::Normal => {
                String::from("A moderate challenge, requiring some skill to overcome.")
            }
            EnemyDifficulty::Hard => String::from("A formidable opponent, testing your abilities."),
            EnemyDifficulty::Boss => String::from("Victory will not come easily."),
        }
    }

    pub fn stats(&self) -> Stats {
        let mut possible_stats = [
            Stat::Power,
            Stat::AttackSpeed,
            Stat::CritMultiplier,
            Stat::CritChance,
            Stat::MaxLife,
            Stat::Block,
            Stat::ParryChance,
            Stat::DodgeChance,
        ];

        let mut rng = thread_rng();

        possible_stats.shuffle(&mut rng);

        match self {
            EnemyDifficulty::Easy => {
                let (life, power, attack_speed) = ENEMY_EASY_STATS;
                let mut stats = Stats::new(life, power, attack_speed);

                let selected_stats = possible_stats.iter().take(ENEMY_EASY_STAT_COUNT);
                self.add_enemy_stats(&mut stats, selected_stats);

                stats
            }
            EnemyDifficulty::Normal => {
                let (life, power, attack_speed) = ENEMY_MEDIUM_STATS;

                let mut stats = Stats::new(life, power, attack_speed);

                let selected_stats = possible_stats.iter().take(ENEMY_MEDIUM_STAT_COUNT);
                self.add_enemy_stats(&mut stats, selected_stats);

                stats
            }
            EnemyDifficulty::Hard => {
                let (life, power, attack_speed) = ENEMY_HARD_STATS;

                let mut stats = Stats::new(life, power, attack_speed);

                let selected_stats = possible_stats.iter().take(ENEMY_HARD_STAT_COUNT);
                self.add_enemy_stats(&mut stats, selected_stats);

                stats
            }
            EnemyDifficulty::Boss => {
                let (life, power, attack_speed) = ENEMY_BOSS_STATS;

                let mut stats = Stats::new(life, power, attack_speed);

                let selected_stats = possible_stats.iter().take(ENEMY_BOSS_STAT_COUNT);
                self.add_enemy_stats(&mut stats, selected_stats);

                stats
            }
        }
    }

    fn add_enemy_stats(&self, stats: &mut Stats, selected_stats: Take<Iter<Stat>>) {
        let mut rng = thread_rng();

        for selected_stat in selected_stats {
            match selected_stat {
                Stat::Power => {
                    stats.power +=
                        rng.gen_range(ENEMY_STAT_POWER_RANGE.0..=ENEMY_STAT_POWER_RANGE.1)
                }
                Stat::AttackSpeed => {
                    stats.attack_speed += rng.gen_range(
                        ENEMY_STAT_ATTACK_SPEED_RANGE.0..=ENEMY_STAT_ATTACK_SPEED_RANGE.1,
                    )
                }
                Stat::CritMultiplier => {
                    stats.crit_multiplier +=
                        rng.gen_range(ENEMY_STAT_CRIT_MULTI_RANGE.0..=ENEMY_STAT_CRIT_MULTI_RANGE.1)
                }
                Stat::CritChance => {
                    stats.crit_chance += rng
                        .gen_range(ENEMY_STAT_CRIT_CHANCE_RANGE.0..=ENEMY_STAT_CRIT_CHANCE_RANGE.1)
                }
                Stat::MaxLife => {
                    stats.max_life +=
                        rng.gen_range(ENEMY_STAT_MAX_LIFE_RANGE.0..=ENEMY_STAT_MAX_LIFE_RANGE.1)
                }
                Stat::Block => {
                    stats.block +=
                        rng.gen_range(ENEMY_STAT_BLOCK_RANGE.0..=ENEMY_STAT_BLOCK_RANGE.1)
                }
                Stat::ParryChance => {
                    stats.parry +=
                        rng.gen_range(ENEMY_STAT_PARRY_CHANCE.0..=ENEMY_STAT_PARRY_CHANCE.1)
                }
                Stat::DodgeChance => {
                    stats.dodge +=
                        rng.gen_range(ENEMY_STAT_DODGE_CHANCE.0..=ENEMY_STAT_DODGE_CHANCE.1)
                }
            }
        }
    }
}
