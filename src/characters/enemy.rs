use colored::ColoredString;

use crate::battle_manager::calculate_damage;

use super::fighter::Fighter;

#[derive(Clone, Debug)]
pub struct Enemy {
    pub name: ColoredString,
    pub description: &'static str,
    pub life: i16,
    pub attack: u16,
}

impl Fighter for Enemy {
    fn name(&self) -> ColoredString {
        self.name.clone()
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

    fn take_damage(&mut self, damage: u16) {
        self.life -= damage as i16;
    }
}
