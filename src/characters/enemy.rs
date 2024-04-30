use crate::characters::fighter::calculate_damage;

use super::fighter::Fighter;
use crate::text_format::TextFormatter;

// TODO change to stats

#[derive(Clone, Debug)]
pub struct Enemy {
    pub name: String,
    pub description: &'static str,
    pub life: i16,
    pub power: u16,
    pub attack_speed: u16,
}

impl Enemy {
    pub fn new(
        name: String,
        description: &'static str,
        life: i16,
        power: u16,
        attack_speed: u16,
    ) -> Enemy {
        Enemy {
            name,
            description,
            life,
            power,
            attack_speed,
        }
    }
}

impl Fighter for Enemy {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn life(&self) -> &i16 {
        &self.life
    }

    fn attack(&self, target: &mut dyn Fighter) -> String {
        let damage = calculate_damage(self.power);
        target.take_damage(damage);

        format!(
            "{} attacks you for {} damage. (Your life: {})",
            &self.name(),
            damage.to_string().text_bold(),
            &target.life().to_string().text_red()
        )
    }

    fn take_damage(&mut self, damage: u16) {
        self.life -= damage as i16;
    }
}
