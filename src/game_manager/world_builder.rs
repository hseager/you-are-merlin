use rand::{seq::SliceRandom, thread_rng};

use crate::{
    config::SIDE_QUEST_COUNT,
    encounter::{Battle, Encounter, MainQuest, Quest, SideQuest},
    enemy::Enemy,
    location::Location,
    theme::{Theme, ThemeLocation},
    utilities::{get_random_array_index, map_text_color},
};

// TODO add boss encounter

pub fn build_world(theme: &Theme) -> Vec<Location> {
    let mut locations = build_locations(&theme);

    create_main_quest(&theme, &mut locations);
    generate_side_quests(&theme, locations, SIDE_QUEST_COUNT)
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

// Fill each location with 3 battle encounters
fn build_encounters(theme_location: &ThemeLocation) -> Vec<Encounter> {
    theme_location
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
        .to_vec()
}

// Add side quests to random locations
fn generate_side_quests(
    theme: &Theme,
    mut locations: Vec<Location>,
    quest_count: usize,
) -> Vec<Location> {
    let mut rng = thread_rng();

    locations.shuffle(&mut rng);

    for location in locations.iter_mut().take(quest_count) {
        let quest = build_side_quest(theme);
        location.encounters.insert(0, quest);
    }

    locations.shuffle(&mut rng);

    locations
}

// Add a the main quest to a random location
fn create_main_quest(theme: &Theme, locations: &mut Vec<Location>) -> () {

    // TODO exclude character from sidequests 
    let quest = Quest::MainQuest(MainQuest {
        character: theme
            .characters
            .get(get_random_array_index(&theme.characters))
            .unwrap(),
    });

    let mut rng = thread_rng();

    locations.shuffle(&mut rng); // TODO group shuffling

    if let Some(location) = locations.get_mut(0) {
        location.encounters.insert(0, Encounter::Quest(quest));
    }
}

fn build_side_quest(theme: &Theme) -> Encounter {
    let side_quest = SideQuest {
        character: theme
            .characters
            .get(get_random_array_index(&theme.characters))
            .unwrap(),
        item: theme
            .items
            .get(get_random_array_index(&theme.items))
            .unwrap(),
    };

    Encounter::Quest(Quest::SideQuest(side_quest))
}
