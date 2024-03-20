use colored::{ColoredString, Colorize};

use crate::theme::load_theme;
use self::entities::Location;

pub mod entities;
mod world_builder;

pub struct GameData {
    pub world_name: &'static str,
    pub player_name: ColoredString,
    pub locations: Vec<Location>,
}

impl GameData {
    pub fn new() -> GameData {
        let theme = load_theme();
        let world_name = theme.world_name;
        let player_name = theme.main_character.bold();
        let locations = world_builder::build_world(theme);
        
        GameData {
            world_name,
            player_name,
            locations
        }
    }
}