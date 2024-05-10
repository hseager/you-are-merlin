use crate::{
    characters::fighter::{calculate_damage, handle_block, is_critical, is_dodge, is_parry},
    config::FIGHTER_BASE_ATTACK_SPEED,
};

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
            crit_chance: 0.0,
            parry: 0.0,
            dodge: 0.0,
        };

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
}
