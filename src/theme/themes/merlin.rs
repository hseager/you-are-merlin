
use crate::theme::{Theme, ThemeEnemy, ThemeLocation};

// Please ChatGPT, fill in the below Rust struct values with the lore from
// Merlin
pub const THEME_DATA: Theme = Theme {
    main_character: "Merlin",
    world_name: "Camelot",
    characters: [
        "Lancelot",
        "Mordred",
        "Arthur Pendragon",
        "Gaius",
        "Guinevere",
        "Uther Pendragon",
    ], // Allied characters to main_character
    boss: ThemeEnemy {
        name: "The Sorceress Morgana",
        description: "A powerful sorceress, skilled in dark magic and harboring deep-seated resentment towards Camelot and those who dwell within it.",
        life: 50,
        attack: 12
    },
    locations: [
        ThemeLocation {
            name: "Camelot Castle",
            description: "A majestic fortress atop rugged cliffs, echoing tales of chivalry.",
            enemies: [
                ThemeEnemy {
                    name: "Black Knight",
                    description: "Clad in dark armor, a formidable foe wielding a deadly sword.",
                    life: 20, // Range 10 - 20
                    attack: 8 // Range 4 - 8
                },
                ThemeEnemy {
                    name: "Enchanted Armor",
                    description: "An animated suit of armor, powered by mystical forces.",
                    life: 25,
                    attack: 6
                },
                ThemeEnemy {
                    name: "Sir Balin's Shade",
                    description: "The lingering spirit of a once noble knight, seeking vengeance.",
                    life: 18,
                    attack: 10
                }
            ]
        },
        ThemeLocation {
            name: "Camelot Tavern",
            description:
                "A lively hub in Camelot's heart, offering hearty meals and merry company.",
            enemies: [
                ThemeEnemy {
                    name: "Rogue Bandit",
                    description: "A skilled thief and fighter, always ready to strike from the shadows.",
                    life: 15,
                    attack: 7
                },
                ThemeEnemy {
                    name: "Rowdy Drunkard",
                    description: "Unpredictable and belligerent, swinging fists with reckless abandon.",
                    life: 12,
                    attack: 5
                },
                ThemeEnemy {
                    name: "Tavern Brawler",
                    description: "A burly fighter, accustomed to barroom brawls and quick to anger.",
                    life: 18,
                    attack: 9
                }
            ]
        },
        ThemeLocation {
            name: "Forest of Balor",
            description:
                "A verdant labyrinth cloaked in ancient mystery, hiding forgotten secrets.",
            enemies: [
                ThemeEnemy {
                    name: "Shadow Spirit",
                    description: "A wraith-like creature, blending with the darkness and striking with ethereal power.",
                    life: 22,
                    attack: 7
                },
                ThemeEnemy {
                    name: "Cursed Druid",
                    description: "A wielder of dark magic, twisted by ancient curses and filled with malice.",
                    life: 20,
                    attack: 8
                },
                ThemeEnemy {
                    name: "Ancient Treant",
                    description: "A towering creature of the forest, ancient and wise yet fiercely protective.",
                    life: 30,
                    attack: 10 
                }
            ]
        },
        ThemeLocation {
            name: "Darkling Woods",
            description:
                "An eerie woods of looming darkness, where shadows dance and secrets whisper.",
            enemies: [
                ThemeEnemy {
                    name: "Shade Stalker",
                    description: "A creature born of shadows, silently hunting its prey with deadly precision.",
                    life: 18,
                    attack: 7
                },
                ThemeEnemy {
                    name: "Wraith Wisp",
                    description: "A flickering apparition, haunting the woods with chilling whispers and ghostly touch.",
                    life: 15,
                    attack: 6
                },
                ThemeEnemy {
                    name: "Nightmare Hound",
                    description: "A monstrous canine, bred from nightmares and fueled by primal instincts.",
                    life: 20,
                    attack: 8
                },
            ]
        },
        ThemeLocation {
            name: "Excalibur's Rest",
            description:
                "A sacred pool where the mighty sword Excalibur was returned to the Lady of the Lake.",
            enemies: [
                ThemeEnemy {
                    name: "Guardian of the Lake",
                    description: "A stalwart defender, bound to protect the sacred waters with unyielding resolve.",
                    life: 25,
                    attack: 8
                },
                ThemeEnemy {
                    name: "Lady's Sentinel",
                    description: "A vigilant guardian, sworn to watch over the resting place of Excalibur.",
                    life: 20,
                    attack: 7
                },
                ThemeEnemy {
                    name: "Holy Warden",
                    description: "A divine warrior, imbued with holy power and righteousness in defense of sacred relics.",
                    life: 30,
                    attack: 10
                },
            ]
        },
        ThemeLocation {
            name: "Dragon's Lair",
            description: "A mysterious cavern rumored to be the home of a fearsome dragon.",
            enemies: [
                ThemeEnemy {
                    name: "Shadow Spirit",
                    description: "A wraith-like creature, blending with the darkness and striking with ethereal power.",
                    life: 22,
                    attack: 8
                },
                ThemeEnemy {
                    name: "Ancient Treant",
                    description: "A towering creature of the forest, ancient and wise yet fiercely protective.",
                    life: 27,
                    attack: 9
                },
                ThemeEnemy {
                    name: "The Great Dragon Kilgharrah",
                    description: "A legendary beast, wreathed in flames and unmatched in strength, guarding its lair with ferocious determination.",
                    life: 30,
                    attack: 10
                },
            ]
        },
    ],
    items: [
        "Morgana's Amulet",
        "Arthur's Polished Armour",
        "Excalibur's Scabbard",
        "Gaius's Potion Vial",
        "Merlin's Dragonlord Staff",
        "Nimueh's Dark Tome",
        "Morgause's Enchanted Dagger",
        "The Cup of Life",
        "The Crystal of Neahtid",
        "The Horn of Cathbhadh",
    ]
};