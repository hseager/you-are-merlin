use crate::{
    game_data::entities::{EnemyDifficulty, LocationType}
};

use self::theme_data::get_themes;

mod theme_data;

#[derive(Clone)]
pub struct Theme {
    pub main_character: &'static str,
    pub world_name: &'static str,
    pub friendly_characters: [&'static str; 6],
    pub locations: [ThemeLocation; 6],
    pub items: [&'static str; 10],
    pub boss: ThemeEnemy,
}

// TODO dirty, redo clones
pub fn load_theme() -> Theme {
    let themes = get_themes();
    let (_name, data) = themes.get(0).expect("Unable to load first theme.");

    data.clone()
}

pub fn get_theme_display_list() -> String {
    get_themes()
        .iter()
        .map(|theme| theme.0.to_string())
        .collect::<Vec<String>>()
        .join(", ")
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
