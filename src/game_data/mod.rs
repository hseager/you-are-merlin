use crate::{text_format::TextFormatter, theme::ThemeItem};

use self::entities::Location;
use crate::theme::Theme;

pub mod entities;
mod world_builder;

// TODO Check GameState and see if we still need this separation of static world data
// or if we can mut locations now that we are cloning everything
pub struct GameData {
    pub world_name: &'static str,
    pub main_character: String,
    pub locations: Vec<Location>,
    pub items: Vec<ThemeItem>,
}

impl GameData {
    pub fn new(theme_data: Theme) -> GameData {
        let theme = theme_data;
        let world_name = theme.world_name;
        let main_character = theme.main_character.text_bold();
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
