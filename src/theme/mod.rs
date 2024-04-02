use colored::Colorize;

use crate::{
    game_data::entities::{EnemyDifficulty, LocationType},
    utilities::map_text_color,
};

use self::theme_data::get_themes;

mod theme_data;

#[derive(Clone, Debug)]
pub struct Theme {
    pub main_character: &'static str,
    pub world_name: &'static str,
    pub friendly_characters: [&'static str; 6],
    pub locations: [ThemeLocation; 6],
    pub items: [&'static str; 10],
    pub boss: ThemeEnemy,
}

pub fn get_theme_display_list() -> String {
    let themes = get_themes();

    let mut joined_themes = String::new();
    for (i, theme) in themes.iter().enumerate() {
        joined_themes.push_str(&theme.0.color(map_text_color(i)).to_string());
        if i != themes.len() - 1 {
            joined_themes.push_str(", ");
        }
    }
    joined_themes
}

pub fn get_theme(input: String) -> Theme {
    let themes = get_themes();
    let theme = themes
        .iter()
        .find(|t| t.0.to_lowercase().contains(&input.to_lowercase()));

    if let Some(theme) = theme {
        theme.1.clone()
    } else {
        get_themes().first().unwrap().1.clone()
    }
}

#[derive(Clone, Debug)]
pub struct ThemeLocation {
    pub name: &'static str,
    pub description: &'static str,
    pub enemies: Option<[ThemeEnemy; 3]>,
    pub class: LocationType,
}

#[derive(Clone, Copy, Debug)]
pub struct ThemeEnemy {
    pub name: &'static str,
    pub description: &'static str,
    pub difficulty: EnemyDifficulty,
}
