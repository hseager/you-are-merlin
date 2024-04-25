use crate::{
    item::{item_builder, EquipableItem},
    text_format::TextFormatter,
};

use self::entities::Location;
use crate::theme::Theme;

pub mod entities;
mod world_builder;

pub struct GameData {
    pub world_name: &'static str,
    pub main_character: String,
    pub locations: Vec<Location>,
    pub items: Vec<EquipableItem>,
}

impl GameData {
    pub fn new(theme_data: Theme) -> GameData {
        let theme = theme_data;
        let world_name = theme.world_name;
        let main_character = theme.main_character.text_bold();
        let items = item_builder::build_items(theme.items.to_vec());
        let locations = world_builder::build_world(theme);

        GameData {
            world_name,
            main_character,
            locations,
            items,
        }
    }
}
