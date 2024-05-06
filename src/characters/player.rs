use crate::{
    characters::fighter::{calculate_damage, handle_block, is_critical, is_dodge, is_parry},
    config::*,
    item::{weapon::Weapon, Equipment, Item, ItemType},
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
            block: PLAYER_BLOCK,
            crit_multiplier: PLAYER_CRIT_MULTI,
            crit_chance: PLAYER_CRIT_CHANCE,
            parry: PLAYER_PARRY_CHANCE,
            dodge: PLAYER_DODGE_CHANCE,
        };

        let fists = Weapon {
            name: String::from("Fists"),
            rarity: crate::item::ItemRarity::Common,
            power: 0,
            attack_speed: 0,
            crit_chance: 0.0,
            crit_multiplier: 0.0,
        };

        let equipment = Equipment {
            armour: None,
            weapon: Some(Box::new(fists)),
            artifact: None,
        };

        Player {
            name,
            stats,
            inventory: Vec::new(),
            equipment,
        }
    }

    // TODO this is pretty horrible, stuck on lifetime errors with references to inventory in ManageEvent
    // Maybe I'll fix this properly on a rainy day :)
    pub fn get_cloned_inventory(&self) -> Vec<Box<dyn Item>> {
        self.inventory.to_vec()
    }

    pub fn add_item_to_inventory(&mut self, item: Box<dyn Item>) {
        self.inventory.push(item);
    }

    pub fn remove_item_to_inventory(&mut self, name: String) {
        let index = self.inventory.iter().position(|i| i.name().eq(&name));
        if let Some(index) = index {
            self.inventory.remove(index);
        }
    }

    pub fn has_item_in_inventory(&self, item: Box<dyn Item>) -> bool {
        self.inventory.iter().any(|i| i.name() == item.name())
    }

    pub fn equip_item(&mut self, item: Box<dyn Item>) {
        match item.item_type() {
            ItemType::Weapon => {
                if let Some(weapon) = &self.equipment.weapon {
                    self.add_item_to_inventory(weapon.clone_box())
                }
                self.equipment.weapon = Some(item);
            }
            ItemType::Armour => {
                if let Some(armour) = &self.equipment.armour {
                    self.add_item_to_inventory(armour.clone_box())
                }
                self.equipment.armour = Some(item);
            }
            ItemType::Artifact => {
                if let Some(artifact) = &self.equipment.artifact {
                    self.add_item_to_inventory(artifact.clone_box())
                }
                self.equipment.artifact = Some(item);
            }
            ItemType::QuestItem => panic!("Can't equip quest items!"),
        }
    }

    pub fn equip_item_by_name(&mut self, item_name: String) {
        let item = self
            .inventory
            .iter()
            .find(|i| {
                println!("{}", i.display_name());
                i.display_name()
                    .to_lowercase()
                    .contains(&item_name.to_lowercase())
            })
            .expect("Unable to get equip item by name");

        let item_name = item.name().clone();

        self.equip_item(item.clone_box());
        self.remove_item_to_inventory(item_name);
    }

    pub fn rest(&mut self) -> String {
        let heal_amount = REST_HEAL_AMOUNT;

        let Stats { life, .. } = self.stats;

        let mut new_life = life + heal_amount;

        if new_life > self.max_life() {
            new_life = self.max_life();
        }

        let heal_amount = new_life - life;

        self.stats.life = new_life;

        format!(
            "Your restore {} life (life: {})",
            heal_amount.to_string().text_bold(),
            new_life.to_string().text_green()
        )
    }
}

impl Fighter for Player {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn life(&self) -> i16 {
        self.stats.life
    }

    fn attack(&mut self, target: &mut dyn Fighter) -> String {
        let mut damage = calculate_damage(self.power());
        let is_crit = is_critical(self.crit_chance());
        let mut action_message = String::from("attack");

        if is_crit {
            action_message = "CRIT".text_green_bold();
            damage = (damage as f32 * self.crit_multiplier()).round() as u16;
        }

        // Handle dodge
        let mut status_text = String::new();
        let is_dodge = is_dodge(target.dodge());
        if is_dodge {
            status_text = String::from("But they dodged! ");
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

            target.take_damage(damage - parry_damage - blocked_damage);
        }

        if !is_dodge {
            if is_parry && blocked_damage > 0 {
                status_text = format!(
                    "They parry and reflect {} damage and block {} damage. ",
                    parry_damage.to_string().text_bold(),
                    blocked_damage.to_string().text_bold()
                );
            } else if is_parry && blocked_damage == 0 {
                status_text = format!(
                    "They parry and reflect {} damage. ",
                    parry_damage.to_string().text_bold(),
                );
            } else if blocked_damage > 0 {
                status_text = format!(
                    "They block {} damage. ",
                    blocked_damage.to_string().text_bold()
                );
            }
        }

        format!(
            "You {} {} for {} damage. {}(Enemy life: {})",
            action_message,
            &target.name(),
            damage.to_string().text_bold(),
            status_text,
            target.life().to_string().text_blue()
        )
    }

    fn take_damage(&mut self, damage: u16) {
        self.stats.life -= damage as i16;
    }

    fn attack_speed_as_milliseconds(&self) -> u16 {
        FIGHTER_BASE_ATTACK_SPEED - (self.attack_speed() * 10)
    }

    fn power(&self) -> u16 {
        let mut power = self.stats.power;

        if let Some(weapon) = &self.equipment.weapon {
            power += weapon.power();
        }

        if let Some(artifact) = &self.equipment.artifact {
            power += artifact.power();
        }

        power
    }

    fn attack_speed(&self) -> u16 {
        let mut attack_speed = self.stats.attack_speed;

        if let Some(weapon) = &self.equipment.weapon {
            attack_speed += weapon.attack_speed();
        }

        if let Some(artifact) = &self.equipment.artifact {
            attack_speed += artifact.attack_speed();
        }

        attack_speed
    }

    fn crit_multiplier(&self) -> f32 {
        let mut crit_multiplier = self.stats.crit_multiplier;

        if let Some(weapon) = &self.equipment.weapon {
            crit_multiplier += weapon.crit_multiplier();
        }

        crit_multiplier
    }

    fn crit_chance(&self) -> f32 {
        let mut crit_chance = self.stats.crit_chance;

        if let Some(weapon) = &self.equipment.weapon {
            crit_chance += weapon.crit_chance();
        }

        crit_chance
    }

    fn max_life(&self) -> i16 {
        let mut max_life = self.stats.max_life;

        if let Some(armour) = &self.equipment.armour {
            max_life += armour.max_life();
        }

        if let Some(artifact) = &self.equipment.artifact {
            max_life += artifact.max_life();
        }

        max_life
    }

    fn block(&self) -> u16 {
        let mut block = self.stats.block;

        if let Some(armour) = &self.equipment.armour {
            block += armour.block();
        }

        block
    }

    fn parry(&self) -> f32 {
        let mut parry = self.stats.parry;

        if let Some(armour) = &self.equipment.armour {
            parry += armour.parry();
        }

        parry
    }

    fn dodge(&self) -> f32 {
        let mut dodge = self.stats.dodge;

        if let Some(armour) = &self.equipment.armour {
            dodge += armour.dodge();
        }

        if let Some(artifact) = &self.equipment.artifact {
            dodge += artifact.dodge();
        }

        dodge
    }
}
