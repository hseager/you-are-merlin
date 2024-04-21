use super::Theme;

mod fallout;
mod lord_of_the_rings;
mod merlin;
mod pokemon;
mod pulp_fiction;
mod star_wars;
mod world_of_warcraft;
mod zelda;

type ThemeTemplate = (&'static str, Theme);

pub fn get_themes() -> Vec<ThemeTemplate> {
    vec![
        ("Merlin", merlin::THEME_DATA),
        ("Zelda", zelda::THEME_DATA),
        ("Lord of the Rings", lord_of_the_rings::THEME_DATA),
        ("Star Wars", star_wars::THEME_DATA),
        ("Pulp Fiction", pulp_fiction::THEME_DATA),
        ("Pokemon", pokemon::THEME_DATA),
        ("World of Warcraft", world_of_warcraft::THEME_DATA),
        ("Fallout", fallout::THEME_DATA),
    ]
}
