use colored::ColoredString;

use crate::battle_manager::calculate_damage;

// TODO change attack to u16
pub trait Fighter {
    fn name(&self) -> &ColoredString;
    fn life(&self) -> &i16;
    fn is_alive(&self) -> bool {
        self.life() > &0
    }
    fn attack(&self, target: &mut dyn Fighter);
    fn take_damage(&mut self, damage: i16);
}

#[derive(Clone, Debug)]
pub struct Enemy {
    pub name: ColoredString,
    pub description: &'static str,
    pub life: i16,
    pub attack: i16,
}

impl Fighter for Enemy {
    fn name(&self) -> &ColoredString {
        &self.name
    }

    fn life(&self) -> &i16 {
        &self.life
    }

    fn attack(&self, target: &mut dyn Fighter) {
        let damage = calculate_damage(self.attack);
        target.take_damage(damage);

        println!(
            "{} attacks you for {} damage. (Your life: {})",
            &self.name(),
            damage,
            &target.life()
        );
    }

    fn take_damage(&mut self, damage: i16) {
        self.life -= damage;
    }
}

pub struct Player<'a> {
    pub name: &'a ColoredString,
    pub max_life: i16,
    pub life: i16,
    pub attack: i16,
}

impl<'a> Fighter for Player<'a> {
    fn name(&self) -> &ColoredString {
        self.name
    }

    fn life(&self) -> &i16 {
        &self.life
    }

    fn attack(&self, target: &mut dyn Fighter) {
        let damage = calculate_damage(self.attack);
        target.take_damage(damage);

        println!(
            "You attack {} for {} damage. (Enemy life: {})",
            &target.name(),
            damage,
            target.life()
        );
    }

    fn take_damage(&mut self, damage: i16) {
        self.life -= damage;
    }
}
