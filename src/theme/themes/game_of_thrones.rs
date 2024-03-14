pub const THEME_DATA: Theme = Theme {
    main_character: "Daenerys Targaryen",
    characters: [
        "Jon Snow",
        "Tyrion Lannister",
        "Arya Stark",
        "Sansa Stark",
        "Cersei Lannister",
        "Jaime Lannister",
    ],
    boss: ThemeEnemy {
        name: "Night King",
        description: "The malevolent leader of the White Walkers, with the power to raise the dead and bring eternal winter.",
        life: 50,
        attack: 12
    },
    locations: [
        ThemeLocation {
            name: "Winterfell",
            description: "The ancestral seat of House Stark, a fortress of stone surrounded by towering walls.",
            enemies: [
                ThemeEnemy {
                    name: "Wight",
                    description: "Reanimated corpses, controlled by the Night King's magic, with an insatiable hunger for the living.",
                    life: 20,
                    attack: 8
                },
                ThemeEnemy {
                    name: "White Walker",
                    description: "The elite soldiers of the Night King, with icy blue eyes and deadly weapons.",
                    life: 25,
                    attack: 6
                },
                ThemeEnemy {
                    name: "Undead Giant",
                    description: "Once a fearsome warrior, now a towering undead monstrosity wielding massive weapons.",
                    life: 18,
                    attack: 10
                }
            ]
        },
        ThemeLocation {
            name: "King's Landing",
            description: "The capital city of the Seven Kingdoms, home to the Iron Throne and the Red Keep.",
            enemies: [
                ThemeEnemy {
                    name: "Gold Cloak",
                    description: "The city watch of King's Landing, armed and armored to enforce the king's peace.",
                    life: 15,
                    attack: 7
                },
                ThemeEnemy {
                    name: "Lannister Soldier",
                    description: "The formidable soldiers of House Lannister, trained in combat and loyal to their queen.",
                    life: 12,
                    attack: 5
                },
                ThemeEnemy {
                    name: "City Guard",
                    description: "Protectors of the Red Keep, sworn to defend the royal family and maintain order in the city.",
                    life: 18,
                    attack: 9
                }
            ]
        },
        ThemeLocation {
            name: "The Wall",
            description: "A colossal ice structure stretching across the northern border of the Seven Kingdoms, manned by the Night's Watch.",
            enemies: [
                ThemeEnemy {
                    name: "Wildling Raider",
                    description: "Fierce warriors from beyond the Wall, seeking to breach the defenses and raid the lands of the south.",
                    life: 20,
                    attack: 8
                },
                ThemeEnemy {
                    name: "Giant",
                    description: "Massive creatures of legend, towering over men and wielding immense strength in battle.",
                    life: 30,
                    attack: 10
                },
                ThemeEnemy {
                    name: "White Walker Scout",
                    description: "Sentries of the Night King, scouting the lands of men for weaknesses and vulnerabilities.",
                    life: 15,
                    attack: 6
                }
            ]
        },
        ThemeLocation {
            name: "Dragonstone",
            description: "A remote island fortress, once the seat of House Targaryen and now the base of operations for Daenerys Targaryen.",
            enemies: [
                ThemeEnemy {
                    name: "Stormcloak",
                    description: "Soldiers of the Stormlands, fiercely loyal to their lords and ready to defend their lands against invaders.",
                    life: 18,
                    attack: 7
                },
                ThemeEnemy {
                    name: "Sellsword Captain",
                    description: "Mercenary leaders, commanding bands of skilled fighters for the right price.",
                    life: 20,
                    attack: 9
                },
                ThemeEnemy {
                    name: "Dragonstone Guard",
                    description: "Defenders of Dragonstone, sworn to protect their lady and her cause with unwavering loyalty.",
                    life: 22,
                    attack: 8
                }
            ]
        },
        ThemeLocation {
            name: "The Eyrie",
            description: "A fortress carved into the side of a mountain, accessible only by a treacherous climb.",
            enemies: [
                ThemeEnemy {
                    name: "Mountain Clansman",
                    description: "Warriors from the hills and valleys of the Vale, fierce and independent.",
                    life: 16,
                    attack: 7
                },
                ThemeEnemy {
                    name: "Sky Cell Keeper",
                    description: "Jailers of the Eyrie, tasked with guarding the prisoners in the perilous sky cells.",
                    life: 14,
                    attack: 6
                },
                ThemeEnemy {
                    name: "Eagle Knight",
                    description: "Elite warriors of House Arryn, masters of aerial combat atop their swift mounts.",
                    life: 20,
                    attack: 8
                }
            ]
        },
        ThemeLocation {
            name: "Pyke",
            description: "The seat of House Greyjoy, a series of towers and bridges built atop rocky spires rising from the sea.",
            enemies: [
                ThemeEnemy {
                    name: "Ironborn Reaver",
                    description: "Raiders and pirates of the Iron Islands, skilled in naval warfare and plundering coastal settlements.",
                    life: 18,
                    attack: 7
                },
                ThemeEnemy {
                    name: "Saltcliffe Archer",
                    description: "Archers from the cliffs and towers of Saltcliffe, raining death upon their enemies from afar.",
                    life: 16,
                    attack: 6
                },
                ThemeEnemy {
                    name: "Drowned Priest",
                    description: "Holy men of the Drowned God, preaching his gospel and leading warriors into battle.",
                    life: 20,
                    attack: 8
                }
            ]
        },
        ThemeLocation {
            name: "Oldtown",
            description: "The oldest city in Westeros, home to the Citadel and the maesters who study there.",
            enemies: [
                ThemeEnemy {
                    name: "Thieves' Guild",
                    description: "Criminals and cutthroats lurking in the shadows, preying upon the unwary and the weak.",
                    life: 15,
                    attack: 7
                },
                ThemeEnemy {
                    name: "Maester's Acolyte",
                    description: "Apprentices of the Citadel, learning the secrets of magic, medicine, and lore under the guidance of their masters.",
                    life: 14,
                    attack: 6
                },
                ThemeEnemy {
                    name: "City Watch",
                    description: "Guardians of the peace in Oldtown, sworn to uphold the law and protect the city from harm.",
                    life: 18,
                    attack: 8
                }
            ]
        },
        // Add more locations as needed
    ],
    items: [
        "Dragonbone Dagger",
        "Greenseer's Weirwood Staff",
        "Wildfire Flask",
        "Faceless Men's Coin",
        "Valyrian Steel Armor",
        "Essence of Nightshade",
        "Ironborn Longship",
        "Night's Watch Cloak",
        "Golden Hand of the King Pin",
        "Dragon Horn",
    ]
};
