pub struct Theme {
    pub main_character: &'static str,
    pub characters: [&'static str; 6],
    pub locations: [ThemeLocation; 6],
    pub items: [&'static str; 10],
}

// Please ChatGPT, fill in the below Rust struct values with the lore from
// Game of Thrones
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
                enemies: [
                    ThemeEnemy {
                        name: "Black Knight",
                        life: 20, // Range 10 - 20
                        attack: 8 // Range 4 - 8
                    },
                    ThemeEnemy {
                        name: "Enchanted Armor",
                        life: 25, // Range 15 - 25
                        attack: 6 // Range 2 - 6
                    },
                    ThemeEnemy {
                        name: "Sir Balin's Shade",
                        life: 18, // Range 8 - 18
                        attack: 10 // Range 5 - 10
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
                        life: 15, // Range 8 - 15
                        attack: 7 // Range 3 - 7
                    },
                    ThemeEnemy {
                        name: "Rowdy Drunkard",
                        life: 12, // Range 5 - 12
                        attack: 5 // Range 2 - 5
                    },
                    ThemeEnemy {
                        name: "Tavern Brawler",
                        life: 18, // Range 10 - 18
                        attack: 9 // Range 4 - 9
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
                        life: 22, // Range 12 - 22
                        attack: 8 // Range 3 - 8
                    },
                    ThemeEnemy {
                        name: "Cursed Druid",
                        life: 20, // Range 10 - 20
                        attack: 7 // Range 4 - 7
                    },
                    ThemeEnemy {
                        name: "Ancient Treant",
                        life: 30, // Range 15 - 30
                        attack: 10 // Range 5 - 10
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
                        life: 18, // Range 10 - 18
                        attack: 7 // Range 3 - 7
                    },
                    ThemeEnemy {
                        name: "Nightmare Hound",
                        life: 20, // Range 12 - 20
                        attack: 8 // Range 4 - 8
                    },
                    ThemeEnemy {
                        name: "Wraith Wisp",
                        life: 15, // Range 8 - 15
                        attack: 6 // Range 2 - 6
                    }
                ]
            },
            ThemeLocation {
                name: "Excalibur's Rest",
                description:
                    "A sacred pool where the mighty sword Excalibur was returned to the Lady of the Lake.",
                enemies: [
                    ThemeEnemy {
                        name: "Guardian of the Lake",
                        life: 25, // Range 15 - 25
                        attack: 8 // Range 4 - 8
                    },
                    ThemeEnemy {
                        name: "Holy Warden",
                        life: 30, // Range 20 - 30
                        attack: 10 // Range 6 - 10
                    },
                    ThemeEnemy {
                        name: "Lady's Sentinel",
                        life: 20, // Range 10 - 20
                        attack: 7 // Range 3 - 7
                    }
                ]
            },
            ThemeLocation {
                name: "Dragon's Lair",
                description: "A mysterious cavern rumored to be the home of a fearsome dragon.",
                enemies: [
                    ThemeEnemy {
                        name: "The Great Dragon Kilgharrah",
                        life: 30, // Range 12 - 30
                        attack: 10 // Range 3 - 10
                    },
                    ThemeEnemy {
                        name: "Shadow Spirit",
                        life: 22, // Range 12 - 22
                        attack: 8 // Range 3 - 8
                    },
                    ThemeEnemy {
                        name: "Ancient Treant",
                        life: 30, // Range 15 - 30
                        attack: 10 // Range 5 - 10
                    }
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
    }
}

#[derive(Clone, Copy)]
pub struct ThemeLocation {
    pub name: &'static str,
    pub description: &'static str,
    pub enemies: [ThemeEnemy; 3],
}

#[derive(Clone, Copy)]
pub struct ThemeEnemy {
    pub name: &'static str,
    pub life: i16,
    pub attack: u8,
}
