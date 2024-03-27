use crate::{
    game_data::entities::{EnemyDifficulty, LocationType},
    theme::*,
};

pub const THEME_DATA: Theme = Theme {
    main_character: "Ash",
    world_name: "Kanto",
    friendly_characters: [
        "Professor Oak",
        "Brock",
        "Misty",
        "Lt. Surge",
        "Erika",
        "Sabrina",
    ],
    boss: ThemeEnemy {
        name: "Giovanni",
        description: "Leader of Team Rocket, a notorious criminal organization, striving to capture rare Pokémon and exploit their power for his nefarious plans.",
        difficulty: EnemyDifficulty::Boss
    },
    locations: [
        ThemeLocation {
            name: "Viridian Forest",
            description: "A dense forest teeming with Bug-type Pokémon, offering a challenging environment for trainers to navigate.",
            class: LocationType::Dungeon("Old Amber"),
            enemies: Some([
                ThemeEnemy {
                    name: "Metapod",
                    description: "Cocoon-like Pokémon capable of hardening their shells to withstand attacks and evolve into Butterfree.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Weedle",
                    description: "Poisonous caterpillar Pokémon often found in forests, capable of evolving into Beedrill.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Pikachu",
                    description: "An Electric-type Pokémon known for its agility and powerful electrical attacks, often favored by trainers for its endearing nature and strong bond with its trainer.",
                    difficulty: EnemyDifficulty::Hard
                },
            ]),
        },
        ThemeLocation {
            name: "Mount Moon",
            description: "A vast cavern system inhabited by Rock-type and Poison-type Pokémon, presenting both geological and biological challenges.",
            class: LocationType::Dungeon("A Moon Stone"),
            enemies: Some([
                ThemeEnemy {
                    name: "Clefairy",
                    description: "A Fairy-type Pokémon known for its whimsical nature and affinity for dancing under the moonlight, often found in the caves of Mount Moon.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Zubat",
                    description: "Bat-like Pokémon known for their echolocation abilities, often encountered in dark cave environments.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Geodude",
                    description: "Rock-type Pokémon with sturdy bodies, capable of rolling at high speeds and unleashing powerful Rock-type moves.",
                    difficulty: EnemyDifficulty::Normal
                },
            ]),
        },
        ThemeLocation {
            name: "Pallet Town",
            description: "A serene coastal town, the starting point for Pokémon trainers, known for its tranquil atmosphere and scenic views.",
            class: LocationType::SafeZone,
            enemies: None,
        },
        ThemeLocation {
            name: "Seafoam Islands",
            description: "A treacherous labyrinth of icy caves and frigid waters, rumored to be the dwelling place of the legendary Pokémon Articuno.",
            class: LocationType::Dungeon("Dome Fossil"),
            enemies: Some([
                ThemeEnemy {
                    name: "Articuno",
                    description: "A legendary Ice/Flying-type Pokémon said to dwell within the depths of the Seafoam Islands, revered for its graceful movements and icy powers.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Seel",
                    description: "Water-type Pokémon often found swimming in the chilly waters of Seafoam Islands, known for their playful demeanor.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Dewgong",
                    description: "A graceful Water/Ice-type Pokémon that inhabits the icy caverns of Seafoam Islands, revered for its beauty and strength.",
                    difficulty: EnemyDifficulty::Hard
                },
            ]),
        },
        ThemeLocation {
            name: "Saffron City Gym",
            description: "A formidable psychic sanctuary located in the heart of Saffron City, where trainers test their mental prowess against the powerful Psychic-type Pokémon of Gym Leader Sabrina.",
            class: LocationType::Dungeon("Marsh Badge"),
            enemies: Some([
                ThemeEnemy {
                    name: "Mr. Mime",
                    description: "Mime Pokémon known for their barrier-making abilities and clever tricks, often employed by Sabrina's trainers to confound opponents.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Kadabra",
                    description: "Highly intelligent Psychic-type Pokémon known for their psychic powers, often found training alongside Sabrina in the Saffron City Gym.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Alakazam",
                    description: "The fully-evolved form of Kadabra, possessing unparalleled psychic abilities and formidable strength, serving as a pinnacle of power in Sabrina's team.",
                    difficulty: EnemyDifficulty::Hard
                },
            ]),
        },
        ThemeLocation {
            name: "Silph Co.",
            description: "A towering skyscraper housing the prestigious Silph Co., known for its cutting-edge technology and the coveted Master Ball.",
            class: LocationType::BossDungeon,
            enemies: Some([
                ThemeEnemy {
                    name: "Kangaskhan",
                    description: "A Normal-type Pokémon known for its fierce maternal instincts and powerful attacks, often found protecting its young in the wild.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Koffing",
                    description: "Poison Gas Pokémon known for emitting toxic fumes, often employed by Team Rocket to create diversions.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Grimer",
                    description: "Sludge Pokémon composed of toxic substances, thriving in polluted environments like the halls of Silph Co.",
                    difficulty: EnemyDifficulty::Normal
                },
            ]),
        },
    ],
    items: [
        "Poke Ball",
        "Great Ball",
        "Ultra Ball",
        "Potion",
        "Super Potion",
        "Hyper Potion",
        "Revive",
        "Rare Candy",
        "TM - Technical Machine",
        "HM - Hidden Machine",
    ]
};
