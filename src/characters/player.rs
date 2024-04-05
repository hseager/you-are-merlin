use colored::ColoredString;

use crate::{battle_manager::calculate_damage, config::REST_HEAL_AMOUNT, items::Item};

use super::fighter::Fighter;

pub struct Player {
    pub name: ColoredString,
    pub max_life: i16,
    pub life: i16,
    pub attack: u16,
    pub inventory: Vec<ColoredString>, // TODO change to vec of items
}

impl Player {
    pub fn add_item_to_inventory(&mut self, item: ColoredString) {
        self.inventory.push(item);
    }

    pub fn has_item_in_inventory(&self, item: &ColoredString) -> bool {
        self.inventory.iter().any(|i| i == item)
    }

    pub fn equip_item(&mut self, item: &Item) {
        self.attack += item.attack;
        self.max_life += item.max_life;
    }

    pub fn heal(&mut self) -> String {
        let heal_amount = REST_HEAL_AMOUNT;
        let mut new_life = self.life + heal_amount;

        if new_life > self.max_life {
            new_life = self.max_life;
        }

        let heal_amount = new_life - self.life;

        self.life = new_life;

        format!(
            "Your life increases by {} (life: {})",
            heal_amount, self.life
        )
    }
}

impl Fighter for Player {
    fn name(&self) -> ColoredString {
        self.name.clone()
    }

    fn life(&self) -> &i16 {
        &self.life
    }

    fn attack(&self, target: &mut dyn Fighter) -> String {
        let damage = calculate_damage(self.attack);
        target.take_damage(damage);

        format!(
            "You attack {} for {} damage. (Enemy life: {})",
            &target.name(),
            damage,
            target.life()
        )
    }

    fn take_damage(&mut self, damage: u16) {
        self.life -= damage as i16;
    }
}
