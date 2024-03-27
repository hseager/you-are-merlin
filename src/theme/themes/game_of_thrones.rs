use crate::game_data::entities::EnemyDifficulty;

use super::*;


pub const THEME_DATA: Theme = Theme {
    main_character: "Jon Snow",
    world_name: "Westeros",
    friendly_characters: [
        "Tyrion Lannister",
        "Arya Stark",
        "Sansa Stark",
        "Cersei Lannister",
        "Jaime Lannister",
        "Daenerys Targaryen",
    ],
    boss: ThemeEnemy {
        name: "The Night King",
        description: "The malevolent leader of the White Walkers, with the power to raise the dead and bring eternal winter.",
        difficulty: EnemyDifficulty::Boss
    },
    locations: [
        ThemeLocation {
            name: "Winterfell",
            description: "The ancestral seat of House Stark, a fortress of stone surrounded by towering walls.",
            class: LocationType::SafeZone,
            enemies: None
        },
        ThemeLocation {
            name: "King's Landing",
            description: "The capital city of the Seven Kingdoms, home to the Iron Throne and the Red Keep.",
            class: LocationType::Dungeon("Wildfire Flask"),
            enemies: Some([
                ThemeEnemy {
                    name: "Gold Cloak",
                    description: "The city watch of King's Landing, armed and armored to enforce the king's peace.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Lannister Soldier",
                    description: "The formidable soldiers of House Lannister, trained in combat and loyal to their queen.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "City Guard",
                    description: "Protectors of the Red Keep, sworn to defend the royal family and maintain order in the city.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        },
        ThemeLocation {
            name: "The Wall",
            description: "A colossal ice structure stretching across the northern border of the Seven Kingdoms, manned by the Night's Watch.",
            class: LocationType::BossDungeon,
            enemies: Some([
                ThemeEnemy {
                    name: "Wildling Raider",
                    description: "Fierce warriors from beyond the Wall, seeking to breach the defenses and raid the lands of the south.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Giant",
                    description: "Massive creatures of legend, towering over men and wielding immense strength in battle.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "White Walker Scout",
                    description: "Sentries of the Night King, scouting the lands of men for weaknesses and vulnerabilities.",
                    difficulty: EnemyDifficulty::Easy
                }
            ])
        },
        ThemeLocation {
            name: "Dragonstone",
            description: "A remote island fortress, once the seat of House Targaryen and now the base of operations for Daenerys Targaryen.",
            class: LocationType::Dungeon("Dragon Horn"),
            enemies: Some([
                ThemeEnemy {
                    name: "Stormcloak",
                    description: "Soldiers of the Stormlands, fiercely loyal to their lords and ready to defend their lands against invaders.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Sellsword Captain",
                    description: "Mercenary leaders, commanding bands of skilled fighters for the right price.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Dragonstone Guard",
                    description: "Defenders of Dragonstone, sworn to protect their lady and her cause with unwavering loyalty.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        },
        ThemeLocation {
            name: "The Eyrie",
            description: "A fortress carved into the side of a mountain, accessible only by a treacherous climb.",
            class: LocationType::Dungeon("Essence of Nightshade"),
            enemies: Some([
                ThemeEnemy {
                    name: "Mountain Clansman",
                    description: "Warriors from the hills and valleys of the Vale, fierce and independent.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Sky Cell Keeper",
                    description: "Jailers of the Eyrie, tasked with guarding the prisoners in the perilous sky cells.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Eagle Knight",
                    description: "Elite warriors of House Arryn, masters of aerial combat atop their swift mounts.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        },
        ThemeLocation {
            name: "Pyke",
            description: "The seat of House Greyjoy, a series of towers and bridges built atop rocky spires rising from the sea.",
            class: LocationType::Dungeon("Faceless Men's Coin"),
            enemies: Some([
                ThemeEnemy {
                    name: "Ironborn Reaver",
                    description: "Raiders and pirates of the Iron Islands, skilled in naval warfare and plundering coastal settlements.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Saltcliffe Archer",
                    description: "Archers from the cliffs and towers of Saltcliffe, raining death upon their enemies from afar.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Drowned Priest",
                    description: "Holy men of the Drowned God, preaching his gospel and leading warriors into battle.",
                    difficulty: EnemyDifficulty::Normal
                }
            ])
        }
    ],
    items: [
        "Heartsbane - Valyrian Steel Sword",
        "Longclaw - Valyrian Steel Sword",
        "Needle - Arya Stark's Sword",
        "Oathkeeper - Valyrian Steel Sword",
        "Ice - Valyrian Steel Greatsword",
        "The Mountain's Plate Armor",
        "The Hound's Helmet",
        "Jaime Lannister's Golden Hand",
        "The Red Woman's Ruby Necklace",
        "The Night King's Ice Spear",
    ]
};
