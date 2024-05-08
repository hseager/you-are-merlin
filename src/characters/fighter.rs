use rand::Rng;

use crate::utilities::roll;

pub trait Fighter {
    fn name(&self) -> String;
    fn life(&self) -> i16;
    fn is_alive(&self) -> bool {
        self.life() > 0
    }
    fn attack(&mut self, target: &mut dyn Fighter) -> String;
    fn take_damage(&mut self, damage: u16);
    fn attack_speed_as_milliseconds(&self) -> u16;
    fn can_attack(&self, current_time: i32, last_attack_time: i32) -> bool {
        current_time - last_attack_time >= self.attack_speed_as_milliseconds() as i32
    }

    fn power(&self) -> u16;
    fn attack_speed(&self) -> u16;
    fn max_life(&self) -> i16;
    fn block(&self) -> u16;
    fn crit_multiplier(&self) -> f32;
    fn crit_chance(&self) -> f32;
    fn parry(&self) -> f32;
    fn dodge(&self) -> f32;
}

// Select random damage from -2 to +2 of current attack stat
pub fn calculate_damage(power: u16) -> u16 {
    let damage_range = 2;
    rand::thread_rng().gen_range(power - damage_range..=power + damage_range)
}

pub fn is_critical(crit_chance: f32) -> bool {
    roll() <= crit_chance
}

pub fn handle_block(damage: u16, block: u16) -> u16 {
    let mut blocked_damage = 0;

    if block > 0 {
        if block >= damage {
            blocked_damage = damage
        } else {
            blocked_damage = block;
        }
    }

    blocked_damage
}

pub fn is_dodge(dodge_chance: f32) -> bool {
    roll() <= dodge_chance
}

pub fn is_parry(parry_chance: f32) -> bool {
    roll() <= parry_chance
}
