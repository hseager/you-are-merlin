use crate::{
    game_data::entities::{EnemyDifficulty, LocationType},
    theme::*,
};

pub const THEME_DATA: Theme = Theme {
    main_character: "Link",
    world_name: "Hyrule",
    friendly_characters: [
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
            class: LocationType::Dungeon("The Spirit Medallion"),
            enemies: Some([
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
            ]),
        },
        ThemeLocation {
            name: "Death Mountain",
            description: "A towering volcano shrouded in smoke and lava, home to the proud Goron tribe and the terrifying Fire Temple.",
            class: LocationType::Dungeon("The Goron's Ruby"),
            enemies: Some([
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
            ]),
        },
        ThemeLocation {
            name: "Zora's Domain",
            description: "A serene waterfall kingdom ruled by the aquatic Zora tribe, famed for their elegant architecture and beautiful music.",
            class: LocationType::SafeZone,
            enemies: None,
        },
        ThemeLocation {
            name: "The Lost Woods",
            description: "An enchanted forest filled with winding paths and mystical illusions, hiding the entrance to the sacred forest temple.",
            class: LocationType::Dungeon("The Forest Medallion"),
            enemies: Some([
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
            ]),
        },
        ThemeLocation {
            name: "Hyrule Field",
            description: "Vast plains stretching across the kingdom of Hyrule, connecting its various regions and landmarks.",
            class: LocationType::Dungeon("The Bunny Hood"),
            enemies: Some([
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
            ]),
        },
        ThemeLocation {
            name: "Ganon's Castle",
            class: LocationType::BossDungeon,
            description: "A foreboding fortress located at the heart of Hyrule, serving as the stronghold of the malevolent Ganondorf, where dark magic and treacherous traps await any who dare to challenge its depths.",
            enemies: Some([
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
            ]),
        },
    ],
    items: [
        ThemeItem {
            name:  "The Master Sword",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "The Hylian Shield",
            item_type: ItemType::Armour
        },
        ThemeItem {
            name:  "The Ocarina of Time",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "The Fairy Bow",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "The Bomb Bag",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "The Hookshot",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "Zora's Tunic",
            item_type: ItemType::Armour
        },
        ThemeItem {
            name:  "The Megaton Hammer",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "The Boomerang",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "The Mirror Shield",
            item_type: ItemType::Armour
        },
    ]
};
