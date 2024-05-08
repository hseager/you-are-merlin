use crate::{game_data::entities::LocationType, theme::*};

/**
 * Please chatGPT, fill this in with Lore from:
 * Lord of the Rings
 * with the following rules:
 * - There should be 6 friendly_characters that doesn't include the main character
 * - There should be 6 locations, with only 1 SafeZone and 1 BossDungeon
 * - Each Dungeon should only have 3 enemies, a mix between Easy, Normal and Hard, but never 3 Hard, and never a Boss.
 * - The Dungeon enum value should be an item reward
 * - The Items array should be 10 items not include Dungeon rewards
 */

pub const THEME_DATA: Theme = Theme {
    main_character: "Frodo Baggins",
    world_name: "Middle-earth",
    friendly_characters: [
        "Samwise Gamgee",
        "Gandalf the Grey",
        "Aragorn",
        "Legolas",
        "Gimli",
        "Galadriel",
    ],
    boss: ThemeEnemy {
        name: "Sauron",
        description: "The Dark Lord of Mordor, seeking the One Ring to rule them all and bring darkness to Middle-earth.",
        difficulty: EnemyDifficulty::Boss
    },
    locations: [
        ThemeLocation {
            name: "The Shire",
            description: "A peaceful land of green hills and hobbit-holes, home to the simple folk of the hobbits.",
            class: LocationType::SafeZone,
            enemies: None
        },
        ThemeLocation {
            name: "Misty Mountains",
            description: "A treacherous mountain range shrouded in mist and snow, home to perilous creatures and ancient secrets.",
            class: LocationType::Dungeon("Mithril Ore"),
            enemies: Some([
                ThemeEnemy {
                    name: "Goblin Scavenger",
                    description: "Scrawny and agile creatures, lurking in the shadows and scavenging for treasures.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Mountain Orc",
                    description: "Brutish and savage warriors, wielding crude weapons and driven by a lust for blood.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Snow Troll",
                    description: "Monstrous beasts of the mountains, with thick fur and sharp claws, hunting for prey.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        },
        ThemeLocation {
            name: "Moria",
            description: "An ancient dwarven kingdom deep beneath the Misty Mountains, now overrun by orcs and goblins.",
            class: LocationType::Dungeon("Mithril Vest"),
            enemies: Some([
                ThemeEnemy {
                    name: "Orc Grunt",
                    description: "Minions of Sauron, fierce and brutal in battle.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Uruk-hai Berserker",
                    description: "Warriors bred by Saruman, wielding great strength and ferocity.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Cave Troll",
                    description: "Massive creatures of ancient legend, wielding deadly clubs and thick armor.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        },
        ThemeLocation {
            name: "Gondor",
            description: "A realm of noble warriors and ancient cities, standing as a bulwark against the darkness of Mordor.",
            class: LocationType::Dungeon("Gondorian Shield"),
            enemies: Some([
                ThemeEnemy {
                    name: "Haradrim Warrior",
                    description: "Warriors from the South, skilled in desert combat and wielding scimitars.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Orc Marauder",
                    description: "Raiders from the East, ruthless and cunning in their attacks.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Nazgûl Knight",
                    description: "Mounted servants of the Dark Lord, clad in black armor and wielding deadly weapons.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        },
        ThemeLocation {
            name: "Helm's Deep",
            description: "A fortress of Rohan, carved into the rock and defended by the brave warriors of the Riddermark.",
            class: LocationType::Dungeon("Horn of Helm Hammerhand"),
            enemies: Some([
                ThemeEnemy {
                    name: "Uruk-hai Berserker",
                    description: "Fierce and relentless warriors bred by Saruman, wielding massive weapons and armored for battle.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "White Hand Captain",
                    description: "Leaders of Saruman's army, skilled tacticians and formidable fighters.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Giant",
                    description: "Massive creatures of legend, towering over men and wreaking havoc with their strength.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        },
        ThemeLocation {
            name: "Mount Doom",
            description: "The fiery mountain in the heart of Mordor, where the One Ring was forged and must be destroyed.",
            class: LocationType::BossDungeon,
            enemies: Some([
                ThemeEnemy {
                    name: "Orc Captain",
                    description: "Leaders of Sauron's armies, cunning and ruthless.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Ringwraith",
                    description: "Servants of the Dark Lord, cloaked in shadows and wielding fear as a weapon.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "The Eye of Sauron",
                    description: "The watchful gaze of the Dark Lord, ever seeking the bearer of the One Ring.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        }
    ],
    items: [
        ThemeItem {
            name:  "Sting - Elven Dagger",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "Andúril - Flame of the West",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "Phial of Galadriel",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "Mithril Shirt",
            item_type: ItemType::Armour
        },
        ThemeItem {
            name:  "The One Ring",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "The Evenstar",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "Orc Chieftain Helmet",
            item_type: ItemType::Armour
        },
        ThemeItem {
            name:  "Durin's Axe",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "Glamdring - Foe-hammer",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "Plated Rohan Greeves",
            item_type: ItemType::Armour
        },
    ]
};
