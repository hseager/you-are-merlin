use crate::{
    game_data::entities::{EnemyDifficulty, LocationType},
    theme::theme_data::THEME_DATA,
};

mod theme_data;

pub struct Theme {
    pub main_character: &'static str,
    pub world_name: &'static str,
    pub friendly_characters: [&'static str; 6],
    pub locations: [ThemeLocation; 6],
    pub items: [&'static str; 10],
    pub boss: ThemeEnemy,
}

pub fn load_theme() -> Theme {
    THEME_DATA
}


// TODO clean up clones
#[derive(Clone)]
pub struct ThemeLocation {
    pub name: &'static str,
    pub description: &'static str,
    pub enemies: Option<[ThemeEnemy; 3]>,
    pub class: LocationType,
}

#[derive(Clone, Copy)]
pub struct ThemeEnemy {
    pub name: &'static str,
    pub description: &'static str,
    pub difficulty: EnemyDifficulty,
}
