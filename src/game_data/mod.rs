use colored::{ColoredString, Colorize};

use self::entities::Location;
use crate::theme::Theme;

pub mod entities;
mod world_builder;

pub struct GameData {
    pub world_name: &'static str,
    pub main_character: ColoredString,
    pub locations: Vec<Location>,
    pub items: Vec<&'static str>,
}

impl GameData {
    pub fn new(theme_data: Theme) -> GameData {
        let theme = theme_data;
        let world_name = theme.world_name;
        let main_character = theme.main_character.bold();
        let items = theme.items.to_vec();
        let locations = world_builder::build_world(theme);

        GameData {
            world_name,
            main_character,
            locations,
            items,
        }
    }
}
