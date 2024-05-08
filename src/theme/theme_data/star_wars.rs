use crate::{game_data::entities::LocationType, theme::*};

pub const THEME_DATA: Theme = Theme {
    main_character: "Luke Skywalker",
    world_name: "The Galaxy",
    friendly_characters: [
        "Princess Leia Organa",
        "Han Solo",
        "Chewbacca",
        "Obi-Wan Kenobi",
        "Yoda",
        "R2-D2",
    ],
    boss: ThemeEnemy {
        name: "Darth Vader",
        description: "The Dark Lord of the Sith, once a Jedi Knight named Anakin Skywalker, who fell to the dark side and became the apprentice of Emperor Palpatine.",
        difficulty: EnemyDifficulty::Boss
    },
    locations: [
        ThemeLocation {
            name: "Death Star",
            description: "An immense space station with the power to destroy entire planets, serving as the ultimate weapon of the Galactic Empire.",
            class: LocationType::BossDungeon,
            enemies: Some([
                ThemeEnemy {
                    name: "Stormtrooper",
                    description: "Elite soldiers of the Galactic Empire, clad in white armor and armed with blaster rifles.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "TIE Fighter Pilot",
                    description: "Skilled pilots flying TIE fighters, engaging enemy starfighters in dogfights with unmatched precision.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Imperial Officer",
                    description: "Officers of the Imperial Navy overseeing the operations of the Death Star, issuing commands with authority.",
                    difficulty: EnemyDifficulty::Normal
                },
            ]),
        },
        ThemeLocation {
            name: "Hoth",
            description: "A frozen wasteland and the location of Echo Base, the secret Rebel Alliance outpost, where the Battle of Hoth took place.",
            class: LocationType::Dungeon("The Khyber Pendant"),
            enemies: Some([
                ThemeEnemy {
                    name: "Snowtrooper",
                    description: "Cold-weather specialists of the Galactic Empire, equipped with insulated armor and heavy blaster rifles.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "AT-AT Walker",
                    description: "Imposing armored vehicles deployed by the Empire, towering over the battlefield and unleashing devastating firepower.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Wampa",
                    description: "Predatory creatures native to Hoth, lurking in the icy caverns and preying on unsuspecting victims.",
                    difficulty: EnemyDifficulty::Easy
                },
            ]),
        },
        ThemeLocation {
            name: "Endor",
            description: "A lush forest moon and the site of the Battle of Endor, where the Rebel Alliance launched a decisive strike against the second Death Star.",
            class: LocationType::Dungeon("The Sarlacc Pit Map"),
            enemies: Some([
                ThemeEnemy {
                    name: "Scout Trooper",
                    description: "Fast-moving reconnaissance troops of the Galactic Empire, patrolling the dense forests of Endor on speeder bikes.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Ewok",
                    description: "Primitive yet resourceful inhabitants of Endor, using guerrilla tactics to ambush Imperial forces and aid the Rebel cause.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Imperial Scout Walker",
                    description: "Lightly armored walkers deployed by the Empire, providing support to ground forces during the battle.",
                    difficulty: EnemyDifficulty::Hard
                },
            ]),
        },
        ThemeLocation {
            name: "Mustafar",
            description: "A volcanic planet with rivers of lava and imposing fortresses, serving as the base of operations for the Sith Lord Darth Vader.",
            class: LocationType::Dungeon("The Sith Holocron"),
            enemies: Some([
                ThemeEnemy {
                    name: "Sith Acolyte",
                    description: "Dark side adepts trained in the ways of the Sith, wielding lightsabers infused with the power of the dark side.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Lava Flea",
                    description: "Small creatures adapted to the extreme heat of Mustafar's lava flows, attacking with fiery bites.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Sith Probe Droid",
                    description: "Remote droids deployed by the Sith, equipped with sensors and blasters to hunt down intruders.",
                    difficulty: EnemyDifficulty::Normal
                },
            ]),
        },
        ThemeLocation {
            name: "Kashyyyk",
            description: "A lush forest planet inhabited by the Wookiee species, known for its towering trees and diverse wildlife.",
            class: LocationType::Dungeon("The Kashyyyk Life Tree Sap"),
            enemies: Some([
                ThemeEnemy {
                    name: "Trandoshan Hunter",
                    description: "Mercenaries hired by the Empire to capture Wookiee slaves, armed with blasters and thermal detonators.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Webweaver Spider",
                    description: "Giant arachnids lurking in the shadowy depths of Kashyyyk's forests, ensnaring prey with sticky webs.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Imperial Scout Trooper",
                    description: "Troopers tasked with scouting and reconnaissance missions, equipped with speeder bikes for rapid mobility.",
                    difficulty: EnemyDifficulty::Normal
                },
            ]),
        },
        ThemeLocation {
            name: "Tatooine",
            description: "A desert planet on the Outer Rim, known for its twin suns, moisture farms, and as the birthplace of Anakin Skywalker.",
            class: LocationType::SafeZone,
            enemies: None,
        },
    ],
    items: [
        ThemeItem {
            name:  "Lightsaber",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "Stormtrooper Blaster Pistol",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "DL-44 Heavy Blaster Pistol",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "Darth Maul's Cape",
            item_type: ItemType::Armour
        },
        ThemeItem {
            name:  "Force Field Generator",
            item_type: ItemType::Armour
        },
        ThemeItem {
            name:  "Holocron",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "Boba Fett's Jetpack",
            item_type: ItemType::Artifact
        },
        ThemeItem {
            name:  "Mandalorian Beskar Armor",
            item_type: ItemType::Armour
        },
        ThemeItem {
            name:  "Thermal Detonator",
            item_type: ItemType::Weapon
        },
        ThemeItem {
            name:  "Comlink",
            item_type: ItemType::Artifact
        },
    ]
    
};
