use crate::{
    characters::fighter::calculate_damage,
    config::{
        FIGHTER_BASE_ATTACK_SPEED, PLAYER_ATTACK, PLAYER_ATTACK_SPEED, PLAYER_LIFE,
        REST_HEAL_AMOUNT,
    },
    item::{
        armour::Armour, artifact::Artifact, quest_item::QuestItem, weapon::Weapon, Equipment, Item,
        ItemType,
    },
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

        let first_weapon = Weapon::new(String::from("Basic Weapon"));
        let first_armour = Armour::new(String::from("Basic Armour"));
        let first_artifact = Artifact::new(String::from("Basic Artifact"));

        let equipment = Equipment {
            armour: Some(Box::new(first_armour)),
            weapon: Some(Box::new(first_weapon)),
            artifact: Some(Box::new(first_artifact)),
        };

        let weapon = Weapon::new(String::from("Needle"));
        let armour = Armour::new(String::from("Wolf Armour"));
        let artifact = Artifact::new(String::from("Onyxia Cape"));
        let quest_item = QuestItem::new(String::from("Orb of Shiver"));

        let test_inventory: Vec<Box<dyn Item>> = vec![
            Box::new(weapon),
            Box::new(armour),
            Box::new(artifact),
            Box::new(quest_item),
        ];

        Player {
            name,
            stats,
            inventory: test_inventory,
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

        let Stats { life, max_life, .. } = self.stats;

        let mut new_life = life + heal_amount;

        if new_life > max_life {
            new_life = max_life;
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

    fn life(&self) -> &i16 {
        &self.stats.life
    }

    fn attack(&self, target: &mut dyn Fighter) -> String {
        let damage = calculate_damage(self.power());
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
}
