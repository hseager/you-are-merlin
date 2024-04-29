use crate::{
    characters::fighter::calculate_damage,
    config::REST_HEAL_AMOUNT,
    item::{Equipment, Item},
};

use super::fighter::Fighter;
use crate::text_format::TextFormatter;

pub struct Player {
    pub name: String,
    pub max_life: i16,
    pub life: i16,
    pub attack: u16,
    pub attack_speed: u16,
    pub inventory: Vec<Box<dyn Item>>,
    pub equipment: Equipment,
}

impl Player {
    pub fn new(name: String, life: i16, attack: u16, attack_speed: u16) -> Player {
        let equipment = Equipment {
            armour: None,
            weapon: None,
            artifact: None,
        };

        Player {
            name,
            max_life: life,
            life,
            attack,
            attack_speed,
            inventory: Vec::new(),
            equipment,
        }
    }

    pub fn add_item_to_inventory(&mut self, item: Box<dyn Item>) {
        self.inventory.push(item);
    }

    pub fn has_item_in_inventory(&self, item: Box<dyn Item>) -> bool {
        self.inventory.iter().any(|i| i.name() == item.name())
    }

    // pub fn equip_item(&mut self, item: &Item) {
    //     self.attack += item.attack;
    //     self.max_life += item.max_life;
    // }

    pub fn rest(&mut self) -> String {
        let heal_amount = REST_HEAL_AMOUNT;
        let mut new_life = self.life + heal_amount;

        if new_life > self.max_life {
            new_life = self.max_life;
        }

        let heal_amount = new_life - self.life;

        self.life = new_life;

        format!(
            "Your restore {} life (life: {})",
            heal_amount.to_string().text_bold(),
            self.life.to_string().text_green()
        )
    }
}

impl Fighter for Player {
    fn name(&self) -> String {
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
            damage.to_string().text_bold(),
            target.life().to_string().text_blue()
        )
    }

    fn take_damage(&mut self, damage: u16) {
        self.life -= damage as i16;
    }
}
