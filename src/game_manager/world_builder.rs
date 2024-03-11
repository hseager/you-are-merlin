use crate::{
    config::QUEST_COUNT,
    encounter::{Battle, Encounter, Quest},
    enemy::Enemy,
    location::Location,
    theme::{Theme, ThemeLocation},
    utilities::{get_random_array_index, map_text_color},
};

pub fn build_world(theme: &Theme) -> Vec<Location> {
    let mut locations = build_locations(&theme);

    generate_quests(&theme, &mut locations, QUEST_COUNT)
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

// TODO this isn't working correctly, too tired to fix rn
fn generate_quests(theme: &Theme, locations: &mut Vec<Location>, quest_count: u8) -> Vec<Location> {
    // Make sure there's always a certain number of quests by blacklisting the random indexes we create
    let mut location_blacklist: Vec<usize> = Vec::new();

    for _i in [0..quest_count] {
        let mut random_location = get_random_array_index(locations);
        let quest = build_quest(&theme);

        if location_blacklist.contains(&random_location) {
            random_location = get_random_array_index(locations); // TODO better unique random stuff
        }

        locations
            .get_mut(random_location)
            .unwrap()
            .encounters
            .insert(0, quest);

        location_blacklist.push(random_location);
    }

    locations.to_vec()
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
