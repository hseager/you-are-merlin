use crate::{
    game_data::entities::{EnemyDifficulty, LocationType},
    theme::*,
};

pub const THEME_DATA: Theme = Theme {
    main_character: "Vincent",
    world_name: "Los Angeles",
    friendly_characters: [
        "Jules",
        "Mia",
        "Butch",
        "Winston Wolfe",
        "Marsellus Wallace",
        "Jimmie",
    ],
    boss: ThemeEnemy {
        name: "Butch",
        description: "A formidable figure in the Los Angeles underworld, Butch is a former boxer navigating a treacherous world of crime and betrayal.",
        difficulty: EnemyDifficulty::Boss
    },
    locations: [
        ThemeLocation {
            name: "Jackrabbit Slim's",
            description: "A retro-themed restaurant where patrons can enjoy a taste of 1950s nostalgia along with a side of danger.",
            class: LocationType::Dungeon("The $5 Shake"),
            enemies: Some([
                ThemeEnemy {
                    name: "Mia's Dealer",
                    description: "Drug dealers lurking in the shadows, peddling illicit substances to unsuspecting patrons.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Butch's Enemy",
                    description: "Rival gang members with a bone to pick, ready to settle scores in the most violent manner possible.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Vincent's Hitman",
                    description: "Professional killers hired to eliminate troublesome targets, lurking in the shadows with deadly intent.",
                    difficulty: EnemyDifficulty::Hard
                },
            ]),
        },
        ThemeLocation {
            name: "Marsellus's Club",
            description: "A lavish nightclub owned by Marsellus Wallace, where the elite come to play and business deals are made under the cover of darkness.",
            class: LocationType::SafeZone,
            enemies: None,
        },
        ThemeLocation {
            name: "The Pawn Shop",
            description: "A seedy establishment dealing in all manner of illegal goods, from stolen merchandise to black market firearms.",
            class: LocationType::Dungeon("The Watch"),
            enemies: Some([
                ThemeEnemy {
                    name: "Pawn Shop Owner",
                    description: "Shady proprietors with connections to the criminal underworld, willing to do anything for the right price.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Zed's Thug",
                    description: "Enforcers working for the mysterious crime lord known only as Zed, enforcing his will with brutal efficiency.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Gimp",
                    description: "Mysterious figures lurking in the shadows, ready to pounce on unsuspecting victims with violent intent.",
                    difficulty: EnemyDifficulty::Hard
                },
            ]),
        },
        ThemeLocation {
            name: "The Diner",
            description: "A greasy spoon diner serving up cheap food and even cheaper coffee, where fate brings together strangers from all walks of life.",
            class: LocationType::Dungeon("The Golden Briefcase"),
            enemies: Some([
                ThemeEnemy {
                    name: "Vincent's Associate",
                    description: "A low-level thug associated with Vincent Vega, often found causing trouble in the diner.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Jules' Associate",
                    description: "Another low-level thug, this one affiliated with Jules Winnfield and usually found stirring up trouble in the diner.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Diner Cook",
                    description: "A disgruntled cook with a short temper, known for wielding kitchen utensils as improvised weapons.",
                    difficulty: EnemyDifficulty::Normal
                },
            ]),
        },
        ThemeLocation {
            name: "Mia's House",
            description: "A stylish suburban home where Mia Wallace resides, hosting extravagant parties and secret rendezvous behind closed doors.",
            class: LocationType::Dungeon("The Royale with Cheese"),
            enemies: Some([
                ThemeEnemy {
                    name: "Drug Dealer",
                    description: "A shady character peddling illicit substances at Mia's parties, leading to potential danger.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Mia's Bodyguard",
                    description: "A hired muscle ensuring the security of Mia Wallace and her guests, willing to use force if necessary.",
                    difficulty: EnemyDifficulty::Hard
                },
                ThemeEnemy {
                    name: "Overdosed Patron",
                    description: "A partygoer who has succumbed to the effects of a drug overdose, unpredictable and potentially dangerous.",
                    difficulty: EnemyDifficulty::Easy
                },
            ]),
        },
        ThemeLocation {
            name: "The Apartment",
            description: "A cramped apartment in the heart of Los Angeles, serving as a hideout for fugitives and a battleground for those seeking revenge.",
            class: LocationType::BossDungeon,
            enemies: Some([
                ThemeEnemy {
                    name: "Hitmen",
                    description: "Professional killers hired to eliminate troublesome targets, lurking in the shadows with deadly intent.",
                    difficulty: EnemyDifficulty::Normal
                },
                ThemeEnemy {
                    name: "Undercover Cops",
                    description: "Law enforcement officers working undercover to bring down criminal enterprises, willing to bend the rules to get the job done.",
                    difficulty: EnemyDifficulty::Easy
                },
                ThemeEnemy {
                    name: "Marsellus's Goons",
                    description: "Hired muscle working for Marsellus Wallace, ready to do whatever it takes to protect their boss's interests.",
                    difficulty: EnemyDifficulty::Hard
                },
            ]),
        },
    ],
    items: [
        "Vincent's 1911 Pistol",
        "Jules' 9mm Pistol",
        "Butch's Katana",
        "Marsellus' Briefcase",
        "Mia's Hypodermic Syringe",
        "Vincent's Wallet",
        "Butch's Brass Knuckles",
        "Marcellus' Gold Watch",
        "Vincent's Leather Jacket",
        "Zed's Chopper",
    ]
};
