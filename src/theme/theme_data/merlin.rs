use crate::{game_data::entities::{EnemyDifficulty, LocationType}, theme::*};

// Please ChatGPT, fill in the below Rust struct values with the lore from
// "Merlin"
pub const THEME_DATA: Theme = Theme {
    main_character: "Merlin",
    world_name: "Camelot",
    friendly_characters: [
        "Lancelot",
        "Mordred",
        "Arthur Pendragon",
        "Gaius",
        "Guinevere",
        "Uther Pendragon",
    ],
    boss: ThemeEnemy {
        name: "The Sorceress Morgana",
        description: "A powerful sorceress, skilled in dark magic and harboring deep-seated resentment towards Camelot and those who dwell within it.",
        difficulty: EnemyDifficulty::Boss
    },
    locations: [
        ThemeLocation {
            name: "Camelot Castle",
            description: "A majestic fortress atop rugged cliffs, echoing tales of chivalry.",
            class: LocationType::Dungeon("Arthur's Polished Armour"),
            enemies: Some([
                ThemeEnemy {
                    name: "Black Knight",
                    description: "Clad in dark armor, a formidable foe wielding a deadly sword.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Enchanted Armor",
                    description: "An animated suit of armor, powered by mystical forces.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Sir Balin's Shade",
                    description: "The lingering spirit of a once noble knight, seeking vengeance.",
                    difficulty: EnemyDifficulty::Normal
                }
            ])
        },
        ThemeLocation {
            name: "Camelot Tavern",
            description:
                "A lively hub in Camelot's heart, offering hearty meals and merry company.",
            class: LocationType::SafeZone,
            enemies: None
        },
        ThemeLocation {
            name: "Forest of Balor",
            description:
                "A verdant labyrinth cloaked in ancient mystery, hiding forgotten secrets.",
            class: LocationType::Dungeon("The Cup of Life"),
            enemies: Some([
                ThemeEnemy {
                    name: "Shadow Spirit",
                    description: "A wraith-like creature, blending with the darkness and striking with ethereal power.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Cursed Druid",
                    description: "A wielder of dark magic, twisted by ancient curses and filled with malice.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Ancient Treant",
                    description: "A towering creature of the forest, ancient and wise yet fiercely protective.",
                    difficulty:  EnemyDifficulty::Hard
                }
            ])
        },
        ThemeLocation {
            name: "Darkling Woods",
            description:
                "An eerie woods of looming darkness, where shadows dance and secrets whisper.",
            class: LocationType::Dungeon("Nimueh's Dark Tome"),
            enemies: Some([
                ThemeEnemy {
                    name: "Shade Stalker",
                    description: "A creature born of shadows, silently hunting its prey with deadly precision.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Wraith Wisp",
                    description: "A flickering apparition, haunting the woods with chilling whispers and ghostly touch.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Nightmare Hound",
                    description: "A monstrous canine, bred from nightmares and fueled by primal instincts.",
                    difficulty: EnemyDifficulty::Hard
                },
            ])
        },
        ThemeLocation {
            name: "Excalibur's Rest",
            description:
                "A sacred pool where the mighty sword Excalibur was returned to the Lady of the Lake.",
            class: LocationType::Dungeon("Excalibur's Scabbard"),
            enemies: Some([
                ThemeEnemy {
                    name: "Guardian of the Lake",
                    description: "A stalwart defender, bound to protect the sacred waters with unyielding resolve.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Lady's Sentinel",
                    description: "A vigilant guardian, sworn to watch over the resting place of Excalibur.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Holy Warden",
                    description: "A divine warrior, imbued with holy power and righteousness in defense of sacred relics.",
                    difficulty: EnemyDifficulty::Hard
                },
            ])
        },
        ThemeLocation {
            name: "Dragon's Lair",
            description: "A mysterious cavern rumored to be the home of a fearsome dragon.",
            class: LocationType::BossDungeon,
            enemies: Some([
                ThemeEnemy {
                    name: "Shadow Spirit",
                    description: "A wraith-like creature, blending with the darkness and striking with ethereal power.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Ancient Treant",
                    description: "A towering creature of the forest, ancient and wise yet fiercely protective.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "The Great Dragon Kilgharrah",
                    description: "A legendary beast, wreathed in flames and unmatched in strength, guarding its lair with ferocious determination.",
                    difficulty: EnemyDifficulty::Hard
                },
            ])
        },
    ],
    items: [
        ThemeItem {
            name:  "Excalibur",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "Morgana's Staff",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "Dragon Scale Armor",
            item_type: ItemType::Armour
        },
        ThemeItem {
            name:  "Gaius's Amulet",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "Mordred's Sword",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "Nimueh's Potion",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "Camelot Shield",
            item_type: ItemType::Armour
        },
        ThemeItem {
            name:  "Merlin's Wand",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "Kilgharrah's Scale",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "Lancelot's Armor",
            item_type: ItemType::Armour
        },
    ]    
};