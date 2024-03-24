// Move boss to boss location.

use crate::game_data::entities::EnemyDifficulty;

use super::*;

// Please ChatGPT, fill in the below Rust struct with the lore from
// "The Legend of Zelda: Ocarina of Time"
pub const THEME_DATA: Theme = Theme {
    main_character: "Link",
    world_name: "Hyrule",
    characters: [
        "Princess Zelda",
        "Navi",
        "Skullkid",
        "Impa",
        "Saria",
        "Ruto",
    ],
    boss: ThemeEnemy {
        name: "Ganon",
        description: "The King of Thieves and the ultimate evil, seeking the power of the Triforce to conquer Hyrule.",
        difficulty: EnemyDifficulty::Boss
    },
    locations: [
        ThemeLocation {
            name: "Gerudo Valley",
            description: "A desert canyon inhabited by the fierce Gerudo tribe, known for their skilled horseback archery and thievery.",
            class: LocationType::Dungeon,
            enemies: [
                ThemeEnemy {
                    name: "Gerudo Guard",
                    description: "Elite warriors of the Gerudo, wielding curved swords and swift horse mounts.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Leevers",
                    description: "Sandy creatures emerging from the ground to attack travelers, making navigation treacherous.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Gerudo Thief",
                    description: "Sneaky bandits lurking in the shadows, attempting to steal valuable items from passersby.",
                    difficulty: EnemyDifficulty::Normal
                },
            ],
        },
        ThemeLocation {
            name: "Death Mountain",
            description: "A towering volcano shrouded in smoke and lava, home to the proud Goron tribe and the terrifying Fire Temple.",
            class: LocationType::Dungeon,
            enemies: [
                ThemeEnemy {
                    name: "Goron Warrior",
                    description: "Mighty rock-eaters wielding massive hammers, rolling down slopes with incredible force.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Fire Keese",
                    description: "Fiery bats swarming around the volcanic caverns, attacking with scorching flames.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Fire Dancer",
                    description: "Agile creatures leaping through the flames, performing deadly dances to summon infernal energy.",
                    difficulty: EnemyDifficulty::Normal
                },
            ],
        },
        ThemeLocation {
            name: "Zora's Domain",
            description: "A serene waterfall kingdom ruled by the aquatic Zora tribe, famed for their elegant architecture and beautiful music.",
            class: LocationType::SafeZone,
            enemies: [
                ThemeEnemy {
                    name: "River Zora",
                    description: "Aquatic monsters lurking in the waters, shooting deadly projectiles with precision accuracy.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Electric Octorok",
                    description: "Electrified cephalopods swimming in the domain's rivers, shocking unsuspecting travelers.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Zora Sentry",
                    description: "Vigilant guards patrolling the domain's boundaries, armed with tridents and keen senses.",
                    difficulty: EnemyDifficulty::Hard
                },
            ],
        },
        ThemeLocation {
            name: "The Lost Woods",
            description: "An enchanted forest filled with winding paths and mystical illusions, hiding the entrance to the sacred forest temple.",
            class: LocationType::Dungeon,
            enemies: [
                ThemeEnemy {
                    name: "Skull Kid",
                    description: "Mischievous spirits playing pranks on travelers, leading them astray with their flute melodies.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Deku Scrub",
                    description: "Small plant-like creatures attacking from the underbrush, spitting nuts with surprising accuracy.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Forest Wolfos",
                    description: "Aggressive wolf-like creatures prowling the woods, attacking with feral ferocity.",
                    difficulty: EnemyDifficulty::Hard
                },
            ],
        },
        ThemeLocation {
            name: "Hyrule Field",
            description: "Vast plains stretching across the kingdom of Hyrule, connecting its various regions and landmarks.",
            class: LocationType::Dungeon,
            enemies: [
                ThemeEnemy {
                    name: "Stalchild",
                    description: "Restless undead emerging from the ground at night, attacking travelers with chilling claws.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Peahat",
                    description: "Flying creatures hovering above the fields, attacking with razor-sharp blades and gusts of wind.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Stalfos Knight",
                    description: "Powerful skeletal warriors rising to challenge those who dare to wander the fields at night.",
                    difficulty: EnemyDifficulty::Hard
                },
            ],
        },
        ThemeLocation {
            name: "Ganon's Castle",
            class: LocationType::BossDungeon,
            description: "A foreboding fortress located at the heart of Hyrule, serving as the stronghold of the malevolent Ganondorf, where dark magic and treacherous traps await any who dare to challenge its depths.",
            enemies: [
                ThemeEnemy {
                    name: "Darknut",
                    description: "Elite warriors clad in heavy armor, wielding massive swords to strike down intruders with swift and powerful attacks.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Wizzrobe",
                    description: "Sorcerous foes conjuring potent spells to assault invaders from afar, disappearing and reappearing in a whirlwind of magic.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Stalfos Knight",
                    description: "Undead knights risen from the depths of the castle, wielding enchanted weapons to defend their master's domain.",
                    difficulty: EnemyDifficulty::Hard
                },
            ],
        },
    ],
    items: [
        "The Master Sword",
        "The Hylian Shield",
        "The Ocarina of Time",
        "The Fairy Bow",
        "The Bomb Bag",
        "The Hookshot",
        "The Boomerang",
        "The Lens of Truth",
        "Some Magic Beans",
        "The Megaton Hammer",
    ],
    quest_items: [
        "The Kokiri's Emerald",
        "The Goron's Ruby",
        "The Zora's Sapphire",
        "The Forest Medallion",
        "The Fire Medallion",
        "The Water Medallion",
        "The Spirit Medallion",
        "The Shadow Medallion",
    ]
};
