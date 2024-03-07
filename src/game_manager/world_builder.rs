use crate::{
    encounter::{Battle, Encounter, Quest},
    enemy::Enemy,
    location::Location,
    theme::{Theme, ThemeLocation},
    utilities::map_text_color,
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
            let enemy: Enemy = Enemy {
                name: enemy.name,
                attack: enemy.attack,
                life: enemy.life,
            };

            let battle: Battle = Battle { enemy };

            Encounter::Battle(battle)
        })
        .to_vec();

    let quest = Quest {
        character: theme.characters.first().unwrap(),
    };

    encounters.append(vec![Encounter::Quest(quest)].as_mut());

    encounters
}
