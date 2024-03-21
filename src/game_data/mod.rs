use colored::{ColoredString, Colorize};

use self::entities::Location;
use crate::theme::load_theme;

pub mod entities;
mod world_builder;

pub struct GameData {
    pub world_name: &'static str,
    pub main_character: ColoredString,
    pub locations: Vec<Location>,
}

impl GameData {
    pub fn new() -> GameData {
        let theme = load_theme();
        let world_name = theme.world_name;
        let main_character = theme.main_character.bold();
        let locations = world_builder::build_world(theme);

        // println!("{:#?}", locations);

        GameData {
            world_name,
            main_character,
            locations,
        }
    }
}
