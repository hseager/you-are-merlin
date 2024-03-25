use colored::Colorize;
use rand::{seq::SliceRandom, thread_rng};

use crate::{
    battle_manager::map_theme_enemy_difficulty_to_stats,
    characters::Enemy,
    theme::{Theme, ThemeLocation},
    utilities::map_text_color,
};

use super::entities::*;

// TODO clean up clones here
pub fn build_world(theme: Theme) -> Vec<Location> {
    let mut characters = theme.friendly_characters.to_vec();
    let mut locations = build_locations(&theme, &mut characters);
    

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

    locations.shuffle(&mut rng);

    locations
}

fn build_locations(theme: &Theme, characters: &mut Vec<&str>) -> Vec<Location> {
    let mut locations = Vec::new();

    for (i, theme_location) in theme.locations.iter().enumerate() {
        locations.push(Location {
            name: theme_location.name.color(map_text_color(i)),
            description: theme_location.description,
            encounters: build_encounters(
                theme,
                theme_location,
                characters
            ),
            class: theme_location.class,
        })
    }
    locations
}

fn build_encounters(theme: &Theme, theme_location: &ThemeLocation, characters: &mut Vec<&str>) -> Vec<Encounter> {
    match theme_location.class {
        LocationType::Dungeon(_) | LocationType::BossDungeon => build_battles(theme_location),
        LocationType::SafeZone => build_side_quest(theme, characters),
    }
}

// Build a side quest for each SafeZone
// Basically what the plan is, for now, is to create 1 sidequest for each SafeZone.
// Once the side quest is complete, remove it from the encounters, and only show the 'explore' action
// when there are encounters in the vec

// TODO fix encounters in safezones
fn build_side_quest(theme: &Theme, characters: &mut Vec<&str>) -> Vec<Encounter> {
    let mut rng = thread_rng();

    assert!(!characters.is_empty(), "No characters available");

    // Choose a random character and remove it from the list to make sure it's unique
    let character = characters.choose_mut(&mut rng).unwrap().to_owned();
    characters.retain(|c| *c != character);

    let dungeons: Vec<_> = theme.locations.iter().filter(|l| {
        if let LocationType::Dungeon(_) = l.class {
            true
        } else {
            false
        }
    }).collect();

    let dungeon = dungeons.choose(&mut rng).expect("Unable to get a dungeon from Theme data when creating side quest.");

    if let LocationType::Dungeon(item) = &dungeon.class {
        vec![Encounter::Quest(Quest::SideQuest(SideQuest {
            character: character.bold(),
            location_name: dungeon.name.bold(), // TODO get same color as location name
            item: item.bold(),
        }))]
    } else {
        panic!("Unexpected class when creating a sidequest. Should be a dungeon");
    }    
}

// Fill each dungeon with 3 battle encounters
fn build_battles(theme_location: &ThemeLocation) -> Vec<Encounter> {
    let mut rng = thread_rng();
    let mut battles = Vec::new();

    let enemies = theme_location.enemies.as_ref().expect("Unable to get enemies when building battles. Check Theme data dungeons have enemies.");

    for enemy in enemies {
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
