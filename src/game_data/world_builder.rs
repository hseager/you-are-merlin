use colored::Colorize;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    characters::Enemy,
    config::{BATTLE_ENCOUNTERS_COUNT, REST_LOCATIONS_COUNT, SIDE_QUEST_COUNT},
    theme::{Theme, ThemeLocation},
    utilities::map_text_color,
};

use super::entities::*;

// TODO clean up clones here
pub fn build_world(theme: Theme) -> Vec<Location> {
    let mut locations = build_locations(&theme);
    let mut characters = theme.characters.to_vec();
    let boss = Enemy {
        name: theme.boss.name.bold().red(),
        description: theme.boss.description,
        life: theme.boss.life,
        attack: theme.boss.attack,
    };

    let mut rng = thread_rng();

    locations.shuffle(&mut rng);

    // We create the boss and mainquest in the first location here so that they are grouped together
    add_boss_encounter(&mut locations, &boss);

    add_quests(
        &theme,
        &mut locations,
        &mut characters,
        &mut theme.quest_items.to_vec(),
        SIDE_QUEST_COUNT,
        boss,
    );

    locations.shuffle(&mut rng);

    add_resting_locations(&mut locations, REST_LOCATIONS_COUNT);

    locations.shuffle(&mut rng);

    locations
}

fn add_quests(
    theme: &Theme,
    locations: &mut [Location],
    characters: &mut Vec<&str>,
    items: &mut Vec<&str>,
    side_quest_count: usize,
    boss: Enemy,
) {
    let mut quests: Vec<Encounter> = Vec::new();

    quests.push(build_main_quest(characters, theme.world_name, boss));

    assert!(
        locations.len() > side_quest_count,
        "Can't have more sidequests than locations. Try adjusting setting in config."
    );
    assert!(
        characters.len() > side_quest_count,
        "Can't have more sidequests than characters. Try adding more characters to Theme data."
    );

    for _ in 0..side_quest_count {
        quests.push(build_side_quest(characters, items));
    }

    for (i, quest) in quests.into_iter().enumerate() {
        let location = locations
            .get_mut(i)
            .expect("Failed to get location when adding quests");
        location.encounters.insert(0, quest);
    }
}

fn build_locations(theme: &Theme) -> Vec<Location> {
    let mut locations = Vec::new();

    for (i, theme_location) in theme.locations.iter().enumerate() {
        locations.push(Location {
            name: theme_location.name.color(map_text_color(i)),
            description: theme_location.description,
            encounters: build_battles(theme_location, BATTLE_ENCOUNTERS_COUNT),
            is_resting_location: false,
        })
    }
    locations
}

// Fill each location with 3 battle encounters
fn build_battles(theme_location: &ThemeLocation, battle_encounter_count: usize) -> Vec<Encounter> {
    let mut rng = thread_rng();
    let mut battles = Vec::new();

    for i in 0..battle_encounter_count {}

    for enemy in theme_location.enemies {
        let battle: Battle = Battle {
            enemy: Enemy {
                name: enemy.name.bold(),
                description: enemy.description,
                attack: enemy.attack,
                life: enemy.life,
            },
        };

        battles.push(Encounter::Battle(battle))
    }

    battles.shuffle(&mut rng);

    battles
}

fn build_main_quest(
    characters: &mut Vec<&str>,
    world_name: &'static str,
    boss: Enemy,
) -> Encounter {
    let mut rng = thread_rng();

    // Choose a random character and remove it from the list to make sure it's unique
    let character = characters.choose_mut(&mut rng).unwrap().to_owned();
    characters.retain(|c| *c != character);

    Encounter::Quest(Quest::MainQuest(MainQuest {
        character: character.bold(),
        world_name,
        boss_name: boss.name,
    }))
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

    Encounter::Quest(Quest::SideQuest(SideQuest {
        character: character.bold(),
        item: item.bold(),
    }))
}

fn add_boss_encounter(locations: &mut [Location], boss: &Enemy) {
    let location = locations.get_mut(0).unwrap();

    location.encounters.push(Encounter::BossFight(Battle {
        enemy: boss.clone(),
    }));
}

fn add_resting_locations(locations: &mut [Location], resting_location_count: usize) {
    assert!(
        locations.len() >= resting_location_count,
        "Can't have more resting locations than locations. Try lowering resting locations in config."
    );

    for i in 0..resting_location_count {
        let location = locations
            .get_mut(i)
            .expect("Unable to get location when adding resting locations");

        location.is_resting_location = true;
    }
}
