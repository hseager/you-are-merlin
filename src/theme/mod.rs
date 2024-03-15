use crate::theme::theme_data::THEME_DATA;

mod theme_data;

pub struct Theme {
    pub main_character: &'static str,
    pub world_name: &'static str,
    pub characters: [&'static str; 6],
    pub locations: [ThemeLocation; 6],
    pub items: [&'static str; 10],
    pub boss: ThemeEnemy,
}

pub fn load_theme() -> Theme {
    THEME_DATA
}

#[derive(Clone, Copy)]
pub struct ThemeLocation {
    pub name: &'static str,
    pub description: &'static str,
    pub enemies: [ThemeEnemy; 3],
}

#[derive(Clone, Copy)]
pub struct ThemeEnemy {
    pub name: &'static str,
    pub description: &'static str,
    pub life: i16,
    pub attack: i16,
}
