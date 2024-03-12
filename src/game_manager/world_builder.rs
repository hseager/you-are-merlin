use rand::{seq::SliceRandom, thread_rng};

use crate::{
    config::QUEST_COUNT,
    encounter::{Battle, Encounter, Quest},
    enemy::Enemy,
    location::Location,
    theme::{Theme, ThemeLocation},
    utilities::{get_random_array_index, map_text_color},
};

// TODO add boss encounter

pub fn build_world(theme: &Theme) -> Vec<Location> {
    let locations = build_locations(&theme);

    generate_quests(&theme, locations, QUEST_COUNT)
}

fn build_locations(theme: &Theme) -> Vec<Location> {
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
            encounters: build_encounters(theme_location),
        })
    }
    locations
}

fn generate_quests(
    theme: &Theme,
    mut locations: Vec<Location>,
    quest_count: usize,
) -> Vec<Location> {
    let mut rng = thread_rng();

    for location in locations.iter_mut().take(quest_count) {
        let quest = build_quest(theme);
        location.encounters.insert(0, quest);
    }

    locations.shuffle(&mut rng);

    locations
}

fn build_encounters(theme_location: &ThemeLocation) -> Vec<Encounter> {
    // Fill encounters with battles by default
    let encounters = theme_location
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
