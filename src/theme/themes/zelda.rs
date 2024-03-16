use crate::theme::{Theme, ThemeEnemy, ThemeLocation};

// Please ChatGPT, fill in the below Rust struct values with the lore from
// "The Legend of Zelda: Ocarina of Time"
pub const THEME_DATA: Theme = Theme {
    main_character: "Link",
    world_name: "Hyrule",
    characters: [
        "Princess Zelda",
        "Navi",
        "Ganondorf",
        "Impa",
        "Saria",
        "Ruto",
    ],
    boss: ThemeEnemy {
        name: "Ganon",
        description: "The King of Thieves and the ultimate evil, seeking the power of the Triforce to conquer Hyrule.",
        life: 50,
        attack: 12,
    },
    locations: [
        ThemeLocation {
            name: "Hyrule Castle",
            description: "A grand fortress standing at the heart of Hyrule, home to the royal family and a symbol of hope.",
            enemies: [
                ThemeEnemy {
                    name: "Stalfos",
                    description: "Undead warriors wielding swords and shields, guarding the castle's corridors.",
                    life: 20,
                    attack: 8,
                },
                ThemeEnemy {
                    name: "Redead",
                    description: "Terrifying creatures emitting paralyzing screams, lurking in the dark corners of the castle.",
                    life: 25,
                    attack: 6,
                },
                ThemeEnemy {
                    name: "Iron Knuckle",
                    description: "Giant knights clad in heavy armor, wielding massive axes and posing a formidable challenge.",
                    life: 18,
                    attack: 10,
                },
            ],
        },
        ThemeLocation {
            name: "Kokiri Forest",
            description: "A tranquil woodland where the Kokiri children live under the protection of the Great Deku Tree.",
            enemies: [
                ThemeEnemy {
                    name: "Deku Baba",
                    description: "Carnivorous plants lurking in the underbrush, attacking with snapping jaws.",
                    life: 15,
                    attack: 7,
                },
                ThemeEnemy {
                    name: "Skulltula",
                    description: "Giant spiders hanging from trees, dropping down to ensnare unsuspecting travelers.",
                    life: 12,
                    attack: 5,
                },
                ThemeEnemy {
                    name: "Wolfos",
                    description: "Fierce wolf-like creatures prowling the forest paths, hunting in packs.",
                    life: 18,
                    attack: 9,
                },
            ],
        },
        // Add more locations here...
    ],
    items: [
        "Master Sword",
        "Hylian Shield",
        "Ocarina of Time",
        "Fairy Bow",
        "Bomb Bag",
        "Hookshot",
        "Boomerang",
        "Lens of Truth",
        "Magic Beans",
        "Megaton Hammer",
    ],
};
