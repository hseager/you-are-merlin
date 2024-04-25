use crate::theme::ThemeItem;

use super::{armour::Armour, artifact::Artifact, weapon::Weapon, EquipableItem, ItemType};

pub fn build_items(items: Vec<ThemeItem>) -> Vec<EquipableItem> {
    items
        .iter()
        .map(|item| match item.item_type {
            ItemType::Weapon => EquipableItem::Weapon(Weapon::new(item.name.to_string())),
            ItemType::Armour => EquipableItem::Armour(Armour::new(item.name.to_string())),
            ItemType::Artifact => EquipableItem::Artifact(Artifact::new(item.name.to_string())),
        })
        .collect()
}
