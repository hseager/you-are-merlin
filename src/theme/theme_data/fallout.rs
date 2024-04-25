use crate::{
    game_data::entities::{EnemyDifficulty, LocationType},
    theme::*,
};

pub const THEME_DATA: Theme = Theme {
    main_character: "a Courier",
    world_name: "Mojave Wasteland",
    friendly_characters: [
        "Doc Mitchell",
        "Sunny Smiles",
        "Veronica Santangelo",
        "Boone",
        "Arcade Gannon",
        "Raul Tejada",
    ],
    boss: ThemeEnemy {
        name: "Legate Lanius",
        description: "The fearsome warlord of Caesar's Legion, renowned for his brutality and unwavering loyalty to Caesar.",
        difficulty: EnemyDifficulty::Boss
    },
    locations: [
        ThemeLocation {
            name: "New Vegas Strip",
            description: "The glittering heart of the Mojave Wasteland, home to opulent casinos and high society.",
            class: LocationType::Dungeon("Helios One Power Grid Key"),
            enemies: Some([
                ThemeEnemy {
                    name: "Securitron",
                    description: "Robotic enforcers of Mr. House, patrolling the streets with deadly efficiency.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Chairman Thug",
                    description: "Members of the Chairman, armed with sleek weapons and a taste for violence.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "White Glove Society",
                    description: "Cultured cannibals lurking in the shadows, hiding their dark appetites behind elegant masks.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        },
        ThemeLocation {
            name: "Goodsprings",
            description: "A peaceful settlement nestled in the desert, where the spirit of community thrives.",
            class: LocationType::SafeZone,
            enemies: None
        },
        ThemeLocation {
            name: "Mojave Outpost",
            description: "A fortified outpost on the edge of civilization, guarding the roads leading into the Mojave Wasteland.",
            class: LocationType::SafeZone,
            enemies: None
        },
        ThemeLocation {
            name: "Hoover Dam",
            description: "A monumental structure, the key to controlling the Mojave's precious water supply and a strategic stronghold.",
            class: LocationType::BossDungeon,
            enemies: Some([
                ThemeEnemy {
                    name: "NCR Trooper",
                    description: "Soldiers of the New California Republic, fighting to maintain order in the wasteland.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Legion Centurion",
                    description: "Elite warriors of Caesar's Legion, ruthless and disciplined in battle.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Fiends",
                    description: "Chem-addicted raiders, wreaking havoc on the wasteland with reckless abandon.",
                    difficulty: EnemyDifficulty::Normal
                }
            ])
        },
        ThemeLocation {
            name: "Hidden Valley",
            description: "A secluded bunker complex, home to the secretive Brotherhood of Steel.",
            class: LocationType::Dungeon("Pip-Boy 3000"),
            enemies: Some([
                ThemeEnemy {
                    name: "Brotherhood Paladin",
                    description: "Knights of the Brotherhood, clad in power armor and wielding advanced weaponry.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Nightkin",
                    description: "Super mutants stealthily stalking their prey, their minds twisted by prolonged Stealth Boy usage.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Scorpion Gulch Radscorpions",
                    description: "Giant mutated scorpions, lurking in the shadows and striking with venomous stings.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        },
        ThemeLocation {
            name: "The Divide",
            description: "A desolate wasteland scarred by cataclysmic events, where the ruins of civilization crumble beneath the weight of history.",
            class: LocationType::Dungeon("Vault 22 Cave Key"),
            enemies: Some([
                ThemeEnemy {
                    name: "Marked Men",
                    description: "Survivors of the Divide's destruction, twisted by radiation and consumed by vengeance.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Tunnelers",
                    description: "Subterranean horrors emerging from the depths, swarming over their prey with relentless aggression.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Ulysses",
                    description: "A lone wanderer, a man of mystery and tragedy, whose actions will shape the fate of the Mojave.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        },
    ],
    items: [
        ThemeItem {
            name:  "10mm Pistol",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "Combat Armor",
            item_type: ItemType::Armour
        },
        ThemeItem {
            name:  "Frag Grenade",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "Plasma Rifle",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "Power Armor",
            item_type: ItemType::Armour
        },
        ThemeItem {
            name:  "Stealth Boy",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "Gatling Laser",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "NCR Ranger Combat Armor",
            item_type: ItemType::Armour
        },
        ThemeItem {
            name:  "Plasma Grenade",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "Super Sledge",
            item_type: ItemType::Weapon
        },
    ]
    
};
