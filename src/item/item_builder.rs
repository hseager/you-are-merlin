use crate::theme::ThemeItem;

use super::{armour::Armour, artifact::Artifact, weapon::Weapon, Item, ItemType};

pub fn build_items(items: &[ThemeItem]) -> Vec<Box<dyn Item>> {
    let mut created_items: Vec<Box<dyn Item>> = Vec::new();

    for item in items.iter() {
        match item.item_type {
            ItemType::Weapon => created_items.push(Box::new(Weapon::new(item.name.to_string()))),
            ItemType::Armour => created_items.push(Box::new(Armour::new(item.name.to_string()))),
            ItemType::Artifact => {
                created_items.push(Box::new(Artifact::new(item.name.to_string())))
            }
            _ => unreachable!(),
        }
    }

    created_items
}
