use crate::{
    characters::fighter::calculate_damage,
    config::{PLAYER_ATTACK, PLAYER_ATTACK_SPEED, PLAYER_LIFE, REST_HEAL_AMOUNT},
    item::{Equipment, Item},
};

use super::{fighter::Fighter, stats::Stats};
use crate::text_format::TextFormatter;

pub struct Player {
    pub name: String,
    pub stats: Stats,
    pub inventory: Vec<Box<dyn Item>>,
    pub equipment: Equipment,
}

impl Player {
    pub fn new(name: String) -> Player {
        let stats = Stats {
            max_life: PLAYER_LIFE,
            life: PLAYER_LIFE,
            power: PLAYER_ATTACK,
            attack_speed: PLAYER_ATTACK_SPEED,
        };

        let equipment = Equipment {
            armour: None,
            weapon: None,
            artifact: None,
        };

        Player {
            name,
            stats,
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

        let Stats {
            mut life, max_life, ..
        } = self.stats;

        let mut new_life = life + heal_amount;

        if new_life > max_life {
            new_life = max_life;
        }

        let heal_amount = new_life - life;

        life = new_life;

        format!(
            "Your restore {} life (life: {})",
            heal_amount.to_string().text_bold(),
            life.to_string().text_green()
        )
    }
}

impl Fighter for Player {
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
            "You attack {} for {} damage. (Enemy life: {})",
            &target.name(),
            damage.to_string().text_bold(),
            target.life().to_string().text_blue()
        )
    }

    fn take_damage(&mut self, damage: u16) {
        self.stats.life -= damage as i16;
    }

    fn attack_speed(&self) -> u16 {
        self.stats.attack_speed
    }
}
