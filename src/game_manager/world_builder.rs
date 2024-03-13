use rand::{seq::SliceRandom, thread_rng};

use crate::{
    config::SIDE_QUEST_COUNT,
    encounter::{Battle, Encounter, MainQuest, Quest, SideQuest},
    enemy::Enemy,
    location::Location,
    theme::{Theme, ThemeLocation},
    utilities::map_text_color,
};

pub fn build_world(theme: &Theme) -> Vec<Location> {
    let mut locations = build_locations(&theme);
    let mut characters = theme.characters.to_vec();
    let mut items = theme.items.to_vec();

    let mut rng = thread_rng();

    locations.shuffle(&mut rng);

    add_quests(
        &mut locations,
        &mut characters,
        &mut items,
        SIDE_QUEST_COUNT,
    );

    locations.shuffle(&mut rng);

    locations
}

fn add_quests(
    locations: &mut Vec<Location>,
    characters: &mut Vec<&str>,
    items: &mut Vec<&str>,
    quest_count: usize,
) {
    let mut quests: Vec<Encounter> = Vec::new();

    quests.push(build_main_quest(characters));

    for _ in 0..quest_count {
        quests.push(build_side_quest(characters, items));
    }

    // TODO very messy, clear up clones and checking, make sure there's always more locations than quests etc
    for (i, location) in locations.iter_mut().take(quests.len()).enumerate() {
        location
            .encounters
            .insert(0, quests.get(i).unwrap().to_owned());
    }

    println!("{:#?}", locations);
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
            encounters: build_battles(theme_location),
        })
    }
    locations
}

// Fill each location with 3 battle encounters
fn build_battles(theme_location: &ThemeLocation) -> Vec<Encounter> {
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

fn build_main_quest(characters: &mut Vec<&str>) -> Encounter {
    let mut rng = thread_rng();

    // TODO add boss encounter

    // Choose a random character and remove it from the list to make sure it's unique
    let character = characters.choose_mut(&mut rng).unwrap().to_owned();
    characters.retain(|c| *c != character);

    Encounter::Quest(Quest::MainQuest(MainQuest { character }))
}

fn build_side_quest(characters: &mut Vec<&str>, items: &mut Vec<&str>) -> Encounter {
    let mut rng = thread_rng();

    // Ensure there are characters and items available
    assert!(!characters.is_empty(), "No characters available");
    assert!(!items.is_empty(), "No items available");

    // Choose a random character and remove it from the list to make sure it's unique
    let character = characters.choose_mut(&mut rng).unwrap().to_owned();
    characters.retain(|c| *c != character);

    // Choose a random item and remove it from the list to make sure it's unique
    let item = items.choose_mut(&mut rng).unwrap().to_owned();
    items.retain(|i| *i != item);

    Encounter::Quest(Quest::SideQuest(SideQuest { character, item }))
}
