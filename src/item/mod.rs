use rand::thread_rng;

use self::{armour::Armour, artifact::Artifact, weapon::Weapon};

mod armour;
mod artifact;
pub mod item_builder;
mod weapon;

#[derive(Clone, Debug)]
pub enum ItemType {
    Armour,
    Weapon,
    Artifact,
}

pub trait Item {
    fn name(&self) -> String;
    fn item_type(&self) -> ItemType;
}

pub struct Equipment {
    pub armour: Option<Armour>,
    pub weapon: Option<Weapon>,
    pub artifact: Option<Artifact>,
}

pub enum EquipableItem {
    Armour(Armour),
    Weapon(Weapon),
    Artifact(Artifact),
}

pub fn get_item(items: &mut Vec<&str>) -> EquipableItem {
    let mut rng = thread_rng();

    let item_name = items.choose_mut(&mut rng).unwrap().to_owned();
    items.retain(|c| *c != item_name);

    let (max_life, attack) = create_item_stats();

    Item {
        name: item_name.text_bold(),
        attack,
        max_life,
    }
}

// pub fn create_item_stats() -> (i16, u16) {
//     let (min_life, max_life) = ITEM_GEN_LIFE;
//     let (min_attack, max_attack) = ITEM_GEN_ATTACK;

//     let max_life = rand::thread_rng().gen_range(min_life..=max_life);
//     let attack: u16 = rand::thread_rng().gen_range(min_attack..=max_attack);

//     (max_life, attack)
// }
