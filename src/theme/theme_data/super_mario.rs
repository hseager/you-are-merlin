use crate::{
    game_data::entities::{EnemyDifficulty, LocationType},
    theme::*,
};

pub const THEME_DATA: Theme = Theme {
    main_character: "Mario",
    world_name: "Mushroom Kingdom",
    friendly_characters: [
        "Luigi",
        "Princess Peach",
        "Yoshi",
        "Toad",
        "Princess Daisy",
        "Toadette",
    ],
    boss: ThemeEnemy {
        name: "Bowser",
        description: "The king of the Koopas, notorious for kidnapping Princess Peach and causing mischief in the Mushroom Kingdom.",
        difficulty: EnemyDifficulty::Boss
    },
    locations: [
        ThemeLocation {
            name: "Mushroom Plains",
            description: "A lush and vibrant area filled with friendly creatures and rolling hills.",
            class: LocationType::SafeZone,
            enemies: None
        },
        ThemeLocation {
            name: "Haunted Mansion",
            description: "A spooky mansion haunted by Boos and other ghostly creatures, rumored to hold hidden treasures.",
            class: LocationType::Dungeon("Poltergust 3000"),
            enemies: Some([
                ThemeEnemy {
                    name: "Boo",
                    description: "Mischievous ghosts that delight in scaring visitors to the mansion.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Dry Bones",
                    description: "Undead Koopa Troopas that reassemble themselves after being defeated.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Big Boo",
                    description: "A giant Boo with immense strength and a relentless pursuit of intruders.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        },
        ThemeLocation {
            name: "Koopa Beach",
            description: "A sunny beach resort inhabited by friendly Koopas and Cheep Cheeps.",
            class: LocationType::Dungeon("Green Shell"),
            enemies: Some([
                ThemeEnemy {
                    name: "Goomba",
                    description: "Small mushroom-like creatures often found wandering around the beach.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Koopa Troopa",
                    description: "Turtle-like creatures that patrol the beach, sometimes hiding inside their shells.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Blooper",
                    description: "Giant squids that emerge from the ocean depths to attack unsuspecting swimmers.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        },
        ThemeLocation {
            name: "Bowser's Castle",
            description: "The imposing fortress of Bowser, filled with traps, lava pits, and his loyal minions.",
            class: LocationType::BossDungeon,
            enemies: Some([
                ThemeEnemy {
                    name: "Koopa Paratroopa",
                    description: "Winged Koopa Troopas that swoop down to attack intruders.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Thwomp",
                    description: "Massive stone blocks that slam down on anyone who dares to pass beneath them.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Bowser Jr.",
                    description: "The mischievous son of Bowser, piloting his own customized airship to thwart Mario's progress.",
                    difficulty: EnemyDifficulty::Normal
                }
            ])
        },
        ThemeLocation {
            name: "Rainbow Road",
            description: "A colorful and chaotic racetrack suspended high above the Mushroom Kingdom, known for its twists, turns, and obstacles.",
            class: LocationType::Dungeon("Starman"),
            enemies: Some([
                ThemeEnemy {
                    name: "Chain Chomp",
                    description: "Enormous metal balls attached to chains, swinging wildly to block the path.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Bullet Bill",
                    description: "High-speed projectiles fired from cannons, targeting racers on the Rainbow Road.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Wiggler",
                    description: "Giant caterpillars that wriggle across the track, causing chaos for racers.",
                    difficulty: EnemyDifficulty::Normal
                }
            ])
        },
        ThemeLocation {
            name: "Piranha Plant Forest",
            description: "A dense forest filled with towering Piranha Plants, their jaws snapping at anything that comes too close.",
            class: LocationType::Dungeon("Super Mushroom"),
            enemies: Some([
                ThemeEnemy {
                    name: "Piranha Plant",
                    description: "Carnivorous plants with sharp teeth and a penchant for devouring unwary travelers.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Fire Bro",
                    description: "Koopa Troopas armed with fireballs, ready to ambush unsuspecting adventurers.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Petey Piranha",
                    description: "A massive Piranha Plant with immense strength and the ability to spew toxic goop.",
                    difficulty: EnemyDifficulty::Hard
                }
            ])
        }
    ],
    items: [
        "Super Mushroom",
        "Fire Flower",
        "Ice Flower",
        "Super Star",
        "1-Up Mushroom",
        "Gold Coin",
        "Power Moon",
        "Bob-omb",
        "Green Shell",
        "Super Leaf",
    ]
};
