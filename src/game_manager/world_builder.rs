use crate::{
    encounter::{Encounter, EncounterType},
    enemy::Enemy,
    location::Location,
    theme::{self, Theme, ThemeLocation},
    utilities::map_text_color,
};

const QUEST_COUNT: u8 = 2;

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
            encounters: build_encounters(theme_location, i, theme),
        })
    }
    locations
}

fn build_encounters(
    theme_location: &ThemeLocation,
    location_index: usize,
    theme: &Theme,
) -> Vec<Encounter> {
    // Fill encounters with each location enemy by default
    let encounters = theme_location
        .enemies
        .map(|enemy| Encounter {
            class: EncounterType::Battle,
            enemy: Enemy {
                name: enemy.name,
                attack: enemy.attack,
                life: enemy.life,
            },
        })
        .to_vec();

    let quest = Encounter {
        class: EncounterType::Quest,
        character: theme.characters.first(),
    }

    encounters.append(quest)
}
