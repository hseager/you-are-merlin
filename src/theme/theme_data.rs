use super::Theme;

mod merlin;
mod zelda;
mod game_of_thrones;
mod star_wars;

type ThemeTemplate = (&'static str, Theme);

pub fn get_themes() -> Vec<ThemeTemplate> {
    vec![
        ("Merlin", merlin::THEME_DATA),
        ("Zelda", zelda::THEME_DATA),
        ("Game Of Thrones", game_of_thrones::THEME_DATA),
        ("Star Wars", star_wars::THEME_DATA)
    ]
}