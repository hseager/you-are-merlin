use crate::{characters::fighter::calculate_damage, config::FIGHTER_BASE_ATTACK_SPEED};

use super::{fighter::Fighter, stats::Stats};
use crate::text_format::TextFormatter;

#[derive(Clone, Debug)]
pub struct Enemy {
    pub name: String,
    pub description: &'static str,
    pub stats: Stats,
}

impl Enemy {
    pub fn new(
        name: String,
        description: &'static str,
        life: i16,
        power: u16,
        attack_speed: u16,
    ) -> Enemy {
        let stats = Stats {
            life,
            max_life: 0,
            power,
            attack_speed,
            block: 0,
            crit_multiplier: 0.0,
        };

        Enemy {
            name,
            description,
            stats,
        }
    }
}

impl Fighter for Enemy {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn life(&self) -> &i16 {
        &self.stats.life
    }

    fn attack(&self, target: &mut dyn Fighter) -> String {
        let damage = calculate_damage(self.stats.power);
        target.take_damage(damage);

        format!(
            "{} attacks you for {} damage. (Your life: {})",
            &self.name(),
            damage.to_string().text_bold(),
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
}
