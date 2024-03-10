use crate::{
    encounter::{Battle, Encounter, Quest},
    enemy::Enemy,
    location::Location,
    theme::{Theme, ThemeLocation},
    utilities::{get_random_array_index, map_text_color},
};

// TODO generate quests
// const QUEST_COUNT: u8 = 2;

pub fn build_world(theme: &Theme) -> Vec<Location> {
    let mut locations = Vec::new();

    for (i, theme_location) in theme.locations.iter().enumerate() {
        let ThemeLocation {
            name, description, ..
        } = theme_location;

        locations.push(Location {
            name,
            description,
            name_color: map_text_color(i),
            current_encounter: 0,
            encounters: build_encounters(theme_location, theme),
        })
    }
    locations
}

fn build_encounters(theme_location: &ThemeLocation, theme: &Theme) -> Vec<Encounter> {
    // Fill encounters with battles by default
    let mut encounters = theme_location
        .enemies
        .map(|enemy| {
            let battle: Battle = Battle {
                enemy: Enemy {
                    name: enemy.name,
                    attack: enemy.attack,
                    life: enemy.life,
                },
            };

            Encounter::Battle(battle)
        })
        .to_vec();

    let quest = build_quest(&theme);

    encounters.insert(0, quest);

    encounters
}

fn build_quest(theme: &Theme) -> Encounter {
    let quest = Quest {
        character: theme
            .characters
            .get(get_random_array_index(&theme.characters))
            .unwrap(),
        item: theme
            .items
            .get(get_random_array_index(&theme.items))
            .unwrap(),
    };

    Encounter::Quest(quest)
}
