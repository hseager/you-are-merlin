use crate::{
    game_data::entities::{EnemyDifficulty, LocationType},
    theme::*,
};

pub const THEME_DATA: Theme = Theme {
    main_character: "Thrall",
    world_name: "Azeroth",
    friendly_characters: [
        "Jaina Proudmoore",
        "King Varian Wrynn",
        "Tyrande Whisperwind",
        "Malfurion Stormrage",
        "Sylvanas Windrunner",
        "Anduin Wrynn",
    ],
    boss: ThemeEnemy {
        name: "Kel'Thuzad",
        description: "The powerful lich and leader of the Scourge forces, serving the will of the Lich King in the icy citadel of Naxxramas.",
        difficulty: EnemyDifficulty::Boss
    },
    locations: [
        ThemeLocation {
            name: "Blackrock Mountain",
            description: "A massive mountain range dominated by the fiery fortress of Ragnaros the Firelord and the dark lair of the Black Dragonflight.",
            class: LocationType::Dungeon("Tome of Tranquilizing Shot"),
            enemies: Some([
                ThemeEnemy {
                    name: "Fire Elemental",
                    description: "Elemental creatures of flame, bound to the will of Ragnaros, wreaking havoc within the Molten Core.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Lava Elemental",
                    description: "Molten creatures born from the depths of the earth, guarding the fiery depths of Blackrock Mountain.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Dark Iron Dwarf",
                    description: "Twisted dwarves enslaved by Ragnaros, wielding fiery weapons and dark magic against intruders.",
                    difficulty: EnemyDifficulty::Normal
                },
            ]),
        },
        ThemeLocation {
            name: "Wailing Caverns",
            description: "A labyrinthine network of caves teeming with corrupted druids and mutated creatures, tainted by the Emerald Nightmare.",
            class: LocationType::Dungeon("Verdant Note"),
            enemies: Some([
                ThemeEnemy {
                    name: "Deviate Ravager",
                    description: "Mutated beasts corrupted by the energies of the Emerald Nightmare, hunting within the Wailing Caverns.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Deviate Python",
                    description: "Serpentine creatures twisted by dark magic, lurking within the shadowy depths of the caverns.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Deviate Shambler",
                    description: "Golems animated by corrupt druids, guarding the secrets of the Fang deep within the Wailing Caverns.",
                    difficulty: EnemyDifficulty::Normal
                },
            ]),
        },
        ThemeLocation {
            name: "Stranglethorn Vale",
            description: "A dense jungle filled with ancient ruins and hostile wildlife, harboring pirates, trolls, and other dangers.",
            class: LocationType::Dungeon("Moss-Twined Heart"),
            enemies: Some([
                ThemeEnemy {
                    name: "Bloodscalp Troll",
                    description: "Savage trolls lurking within the dense jungles of Stranglethorn Vale, known for their brutality and dark rituals.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Bloodsail Pirate",
                    description: "Ruthless pirates sailing the waters of Stranglethorn Vale, preying upon unwary travelers and rival factions.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Panther",
                    description: "Stealthy predators stalking the shadows of Stranglethorn Vale, hunting for prey amidst the dense foliage.",
                    difficulty: EnemyDifficulty::Easy
                },
            ]),
        },
        ThemeLocation {
            name: "Orgrimmar",
            description: "The bustling capital city of the Horde, nestled within the arid landscape of central Kalimdor.",
            class: LocationType::SafeZone,
            enemies: None,
        },        
        ThemeLocation {
            name: "The Deadmines",
            description: "A sprawling network of tunnels and caverns beneath Westfall, transformed into a stronghold by the Defias Brotherhood.",
            class: LocationType::Dungeon("Defias Orders"),
            enemies: Some([
                ThemeEnemy {
                    name: "Defias Pillager",
                    description: "Bandits and thieves serving the Defias Brotherhood, plundering the countryside and terrorizing the inhabitants of Westfall.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy
                {
                    name: "Defias Enforcer",
                    description: "Brutal enforcers of the Defias Brotherhood, wielding heavy weapons and intimidating their foes with brute strength.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Defias Miner",
                    description: "Laborers and workers conscripted by the Defias Brotherhood, toiling in the depths of the Deadmines and guarding their ill-gotten treasures.",
                    difficulty: EnemyDifficulty::Easy
                },
            ]),
        },
        ThemeLocation {
            name: "Naxxramas",
            description: "A massive necropolis floating above the Eastern Plaguelands, serving as the seat of the powerful lich Thaddius and his Scourge forces.",
            class: LocationType::BossDungeon,
            enemies: Some([
                ThemeEnemy {
                    name: "Patchwerk",
                    description: "A monstrous abomination created by the Scourge, serving as one of the guardians of Naxxramas.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Grobbulus",
                    description: "A flesh giant stitched together from the remains of countless corpses, unleashed by Thaddius to defend Naxxramas.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Thaddius",
                    description: "A powerful monstrosity formed from the fusion of two flesh giants, wielding devastating electrical powers against intruders.",
                    difficulty: EnemyDifficulty::Boss
                },
            ]),
        },        
    ],
    items: [
        "The Ashbringer",
        "Thunderfury, Blessed Blade of the Windseeker",
        "Sulfuras, Hand of Ragnaros",
        "Atiesh, Greatstaff of the Guardian",
        "Shadowfang",
        "Teebu's Blazing Longsword",
        "Warden's Staff",
        "Arugal's Fist",
        "Quel'Serrar",
        "Ironfoe",
    ]
};
