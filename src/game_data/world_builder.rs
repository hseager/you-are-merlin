use colored::Colorize;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    battle_manager::map_theme_enemy_difficulty_to_stats,
    characters::Enemy,
    config::SIDE_QUEST_COUNT,
    theme::{Theme, ThemeLocation},
    utilities::map_text_color,
};

use super::entities::*;

// TODO clean up clones here
pub fn build_world(theme: Theme) -> Vec<Location> {
    let mut locations = build_locations(&theme);
    let mut characters = theme.friendly_characters.to_vec();

    let (boss_life, boss_attack) = map_theme_enemy_difficulty_to_stats(theme.boss.difficulty);
    let boss = Enemy {
        name: theme.boss.name.bold().red(),
        description: theme.boss.description,
        life: boss_life,
        attack: boss_attack,
    };

    let mut rng = thread_rng();

    locations.shuffle(&mut rng);

    add_main_quest(&mut locations, boss, &mut characters, theme.world_name);

    // add_side_quests(
    //     &mut locations,
    //     &mut characters,
    //     &mut theme.quest_items.to_vec(),
    //     SIDE_QUEST_COUNT,
    // );

    locations.shuffle(&mut rng);

    locations
}

// fn add_side_quests(
//     locations: &mut [Location],
//     characters: &mut Vec<&str>,
//     items: &mut Vec<&str>,
//     side_quest_count: usize,
// ) {
//     let mut quests: Vec<Encounter> = Vec::new();

//     assert!(
//         locations.len() > side_quest_count,
//         "Can't have more sidequests than locations. Try adjusting setting in config."
//     );
//     assert!(
//         characters.len() > side_quest_count,
//         "Can't have more sidequests than characters. Try adding more characters to Theme data."
//     );

//     for _ in 0..side_quest_count {
//         quests.push(build_side_quest(characters, items));
//     }

//     for (i, quest) in quests.into_iter().enumerate() {
//         let location = locations
//             .get_mut(i)
//             .expect("Failed to get location when adding quests");
//         location.encounters.insert(0, quest);
//     }
// }

fn build_locations(theme: &Theme) -> Vec<Location> {
    let mut locations = Vec::new();

    for (i, theme_location) in theme.locations.iter().enumerate() {
        locations.push(Location {
            name: theme_location.name.color(map_text_color(i)),
            description: theme_location.description,
            encounters: build_encounters(
                theme_location,
                theme.friendly_characters,
                theme.quest_items,
            ),
            class: theme_location.class,
        })
    }
    locations
}

fn build_encounters(theme_location: &ThemeLocation, characters) -> Vec<Encounter> {
    match theme_location.class {
        LocationType::Dungeon | LocationType::BossDungeon => build_battles(theme_location),
        LocationType::SafeZone => build_side_quests(characters, items),
    }
}

// Fill each dungeon with 3 battle encounters
fn build_battles(theme_location: &ThemeLocation) -> Vec<Encounter> {
    let mut rng = thread_rng();
    let mut battles = Vec::new();

    for enemy in theme_location.enemies {
        let (life, attack) = map_theme_enemy_difficulty_to_stats(enemy.difficulty);
        let battle: Battle = Battle {
            enemy: Enemy {
                name: enemy.name.bold(),
                description: enemy.description,
                life,
                attack,
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
    boss: &Enemy,
) -> Encounter {
    let mut rng = thread_rng();

    // Choose a random character and remove it from the list to make sure it's unique
    let character = characters.choose_mut(&mut rng).unwrap().to_owned();
    characters.retain(|c| *c != character);

    Encounter::Quest(Quest::MainQuest(MainQuest {
        character: character.bold(),
        world_name,
        boss_name: boss.name.clone(),
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

fn add_main_quest(
    locations: &mut [Location],
    boss: Enemy,
    characters: &mut Vec<&str>,
    world_name: &'static str,
) {
    let location = locations
        .iter_mut()
        .find(|l| l.class == LocationType::BossDungeon)
        .expect("Unable to find BossDungeon. Make sure at least one Theme location is a LocationType::BossLocation.");

    location
        .encounters
        .insert(0, build_main_quest(characters, world_name, &boss));

    location
        .encounters
        .push(Encounter::BossFight(Battle { enemy: boss }));
}
