pub struct Theme {
    pub main_character: &'static str,
    pub characters: [&'static str; 6],
    pub locations: [ThemeLocation; 6],
    pub items: [&'static str; 10],
    pub enemies: [ThemeEnemy; 6],
}

// Please ChatGPT, fill in the below Rust struct values with the lore from
// Merlin
pub fn load_theme() -> Theme {
    Theme {
        main_character: "Merlin",
        characters: [
            "Morgana",
            "Mordred",
            "Arthur Pendragon",
            "Gaius",
            "Guinevere (Gwen)",
            "Uther Pendragon",
        ], // Excluding main_character
        locations: [
            ThemeLocation {
                name: "Camelot Castle",
                description: "A majestic fortress atop rugged cliffs, echoing tales of chivalry.",
            },
            ThemeLocation {
                name: "Camelot Tavern",
                description:
                    "A lively hub in Camelot's heart, offering hearty meals and merry company.",
            },
            ThemeLocation {
                name: "Forest of Balor",
                description:
                    "A verdant labyrinth cloaked in ancient mystery, hiding forgotten secrets.",
            },
            ThemeLocation {
                name: "Darkling Woods",
                description:
                    "An eerie woods of looming darkness, where shadows dance and secrets whisper.",
            },
            ThemeLocation {
                name: "Dragon's Lair",
                description: "A mysterious cavern rumored to be the home of a fearsome dragon.",
            },
            ThemeLocation {
                name: "Excalibur's Rest",
                description: "A sacred pool where the mighty sword Excalibur was returned to the Lady of the Lake.",
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
        ],
        enemies: [ // TODO Maybe move life & attack to building game world
            ThemeEnemy {
                name: "Morgana's Shadow Assassin",
                life: 18,  // Range 12 - 24
                attack: 6, // Range 4 - 8
            },
            ThemeEnemy {
                name: "Uther's Undead Knight",
                life: 20,
                attack: 5,
            },
            ThemeEnemy {
                name: "Nimueh's Water Elemental",
                life: 17,
                attack: 7,
            },
            ThemeEnemy {
                name: "The Great Dragon Kilgharrah",
                life: 22,
                attack: 8,
            },
            ThemeEnemy {
                name: "The Dorocha",
                life: 19,
                attack: 5,
            },
            ThemeEnemy {
                name: "A Knight of Medhir",
                life: 21,
                attack: 7,
            },
        ],
    }
}

pub struct ThemeLocation {
    pub name: &'static str,
    pub description: &'static str,
}

pub struct ThemeEnemy {
    pub name: &'static str,
    pub life: i16,
    pub attack: i8,
}
