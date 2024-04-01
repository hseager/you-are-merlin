use std::{thread::sleep, time::Duration};

use colored::ColoredString;

use crate::{battle_manager::calculate_damage, config::REST_INTERVAL_SECONDS, items::Item};

pub trait Fighter {
    fn name(&self) -> ColoredString;
    fn life(&self) -> &i16;
    fn is_alive(&self) -> bool {
        self.life() > &0
    }
    fn attack(&self, target: &mut dyn Fighter);
    fn take_damage(&mut self, damage: u16);
}

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

    pub fn heal(&mut self) {
        while self.life < self.max_life {
            let heal_amount = 10;
            let mut new_life = self.life + heal_amount;
            if new_life > self.max_life {
                new_life = self.max_life;
            }

            println!(
                "Your life increases by {} (life: {})",
                new_life - self.life,
                new_life
            );
            self.life = new_life;
            sleep(Duration::from_secs(REST_INTERVAL_SECONDS as u64));
        }
    }
}

impl Fighter for Player {
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
            "You attack {} for {} damage. (Enemy life: {})",
            &target.name(),
            damage,
            target.life()
        );
    }

    fn take_damage(&mut self, damage: u16) {
        self.life -= damage as i16;
    }
}
