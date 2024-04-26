use self::{armour::Armour, artifact::Artifact, weapon::Weapon};

mod armour;
mod artifact;
pub mod item_builder;
mod weapon;

pub trait Item {
    fn name(&self) -> String;
    fn item_type(&self) -> ItemType;
    fn display_stats(&self) -> String;
}

#[derive(Clone, Debug)]
pub enum ItemType {
    Armour,
    Weapon,
    Artifact,
}

pub struct Equipment {
    pub armour: Option<Armour>,
    pub weapon: Option<Weapon>,
    pub artifact: Option<Artifact>,
}
