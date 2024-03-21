use crate::theme::{Theme, ThemeEnemy, ThemeLocation};

// TODO probably best to calculate life and attack in game rather than theme, easy, medium, hard enemies

// Please ChatGPT, fill in the below Rust struct values with the lore from
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
        life: 50,
        attack: 12,
    },
    locations: [
        ThemeLocation {
            name: "Gerudo Valley",
            description: "A desert canyon inhabited by the fierce Gerudo tribe, known for their skilled horseback archery and thievery.",
            enemies: [
                ThemeEnemy {
                    name: "Gerudo Guard",
                    description: "Elite warriors of the Gerudo, wielding curved swords and swift horse mounts.",
                    life: 22,
                    attack: 8,
                },
                ThemeEnemy {
                    name: "Leevers",
                    description: "Sandy creatures emerging from the ground to attack travelers, making navigation treacherous.",
                    life: 20,
                    attack: 7,
                },
                ThemeEnemy {
                    name: "Gerudo Thief",
                    description: "Sneaky bandits lurking in the shadows, attempting to steal valuable items from passersby.",
                    life: 18,
                    attack: 9,
                },
            ],
        },
        ThemeLocation {
            name: "Death Mountain",
            description: "A towering volcano shrouded in smoke and lava, home to the proud Goron tribe and the terrifying Fire Temple.",
            enemies: [
                ThemeEnemy {
                    name: "Goron Warrior",
                    description: "Mighty rock-eaters wielding massive hammers, rolling down slopes with incredible force.",
                    life: 25,
                    attack: 10,
                },
                ThemeEnemy {
                    name: "Fire Keese",
                    description: "Fiery bats swarming around the volcanic caverns, attacking with scorching flames.",
                    life: 15,
                    attack: 7,
                },
                ThemeEnemy {
                    name: "Fire Dancer",
                    description: "Agile creatures leaping through the flames, performing deadly dances to summon infernal energy.",
                    life: 18,
                    attack: 8,
                },
            ],
        },
        ThemeLocation {
            name: "Zora's Domain",
            description: "A serene waterfall kingdom ruled by the aquatic Zora tribe, famed for their elegant architecture and beautiful music.",
            enemies: [
                ThemeEnemy {
                    name: "River Zora",
                    description: "Aquatic monsters lurking in the waters, shooting deadly projectiles with precision accuracy.",
                    life: 20,
                    attack: 9,
                },
                ThemeEnemy {
                    name: "Electric Octorok",
                    description: "Electrified cephalopods swimming in the domain's rivers, shocking unsuspecting travelers.",
                    life: 18,
                    attack: 7,
                },
                ThemeEnemy {
                    name: "Zora Sentry",
                    description: "Vigilant guards patrolling the domain's boundaries, armed with tridents and keen senses.",
                    life: 22,
                    attack: 8,
                },
            ],
        },
        ThemeLocation {
            name: "Lost Woods",
            description: "An enchanted forest filled with winding paths and mystical illusions, hiding the entrance to the sacred forest temple.",
            enemies: [
                ThemeEnemy {
                    name: "Skull Kid",
                    description: "Mischievous spirits playing pranks on travelers, leading them astray with their flute melodies.",
                    life: 18,
                    attack: 7,
                },
                ThemeEnemy {
                    name: "Deku Scrub",
                    description: "Small plant-like creatures attacking from the underbrush, spitting nuts with surprising accuracy.",
                    life: 15,
                    attack: 6,
                },
                ThemeEnemy {
                    name: "Forest Wolfos",
                    description: "Aggressive wolf-like creatures prowling the woods, attacking with feral ferocity.",
                    life: 20,
                    attack: 8,
                },
            ],
        },
        ThemeLocation {
            name: "Hyrule Field",
            description: "Vast plains stretching across the kingdom of Hyrule, connecting its various regions and landmarks.",
            enemies: [
                ThemeEnemy {
                    name: "Stalchild",
                    description: "Restless undead emerging from the ground at night, attacking travelers with chilling claws.",
                    life: 15,
                    attack: 7,
                },
                ThemeEnemy {
                    name: "Peahat",
                    description: "Flying creatures hovering above the fields, attacking with razor-sharp blades and gusts of wind.",
                    life: 18,
                    attack: 8,
                },
                ThemeEnemy {
                    name: "Stalfos Knight",
                    description: "Powerful skeletal warriors rising to challenge those who dare to wander the fields at night.",
                    life: 22,
                    attack: 9,
                },
            ],
        },
        ThemeLocation {
            name: "Hyrule Castle Town",
            description: "A bustling hub of commerce and culture within the shadow of Hyrule Castle, where travelers from all corners of the kingdom converge.",
            enemies: [
                ThemeEnemy {
                    name: "Poe",
                    description: "Ghostly spirits haunting the streets at night, stealing the souls of unsuspecting victims.",
                    life: 20,
                    attack: 8,
                },
                ThemeEnemy {
                    name: "Gossip Stone",
                    description: "Mysterious stone monuments scattered throughout the town, revealing secrets to those who listen.",
                    life: 12,
                    attack: 5,
                },
                ThemeEnemy {
                    name: "Town Guard",
                    description: "Vigilant soldiers patrolling the streets, maintaining order and protecting the town's inhabitants.",
                    life: 25,
                    attack: 10,
                },
            ],
        },        
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
