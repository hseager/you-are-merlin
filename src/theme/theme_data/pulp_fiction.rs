use crate::{game_data::entities::LocationType, theme::*};

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
            class: LocationType::Dungeon("A Royale with Cheese"),
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
    ThemeItem {
        name:  "Vincent's Revolver",
        item_type: ItemType::Weapon
    },
    ThemeItem {
        name:  "Butch's Katana",
        item_type: ItemType::Weapon
    },
    ThemeItem {
        name:  "Marvin's Boots",
        item_type: ItemType::Armour
    },
    ThemeItem {
        name:  "Marsellus' Briefcase",
        item_type: ItemType::Artifact
    },
    ThemeItem {
        name:  "Jules's Wallet",
        item_type: ItemType::Artifact
    },
    ThemeItem {
        name:  "Winston Wolf's Suit",
        item_type: ItemType::Armour
    },
    ThemeItem {
        name:  "Zed's Chopper",
        item_type: ItemType::Weapon
    },
    ThemeItem {
        name:  "The Gold Watch",
        item_type: ItemType::Artifact
    },
    ThemeItem {
        name:  "Butch's Brass Knuckles",
        item_type: ItemType::Weapon
    },
    ThemeItem {
        name:  "Pumpkin's Leather Jacket",
        item_type: ItemType::Armour
    },
    ThemeItem {
        name: "Vincent's Zippo Lighter",
        item_type: ItemType::Artifact
    },ThemeItem {
        name: "Winston Wolf's Business Card",
        item_type: ItemType::Artifact
    },
    ThemeItem {
        name: "The Gimp Suit",
        item_type: ItemType::Armour
    },ThemeItem {
        name: "Vincent's MAC-10",
        item_type: ItemType::Weapon
    },ThemeItem {
        name: "Jules's Nickel-Plated Colt .45",
        item_type: ItemType::Weapon
    },ThemeItem {
        name: "Mia's Gold-Plated Colt Cobra",
        item_type: ItemType::Weapon
    },ThemeItem {
        name: "Zed's S&M Ball Gag",
        item_type: ItemType::Weapon
    },
    ThemeItem {
        name: "Mia's Sequined Disco Dress",
        item_type: ItemType::Armour
    },
    ThemeItem {
        name: "Jules's Bulletproof Vest",
        item_type: ItemType::Armour
    },
    ThemeItem {
        name: "Jimmie's Plaid Bathrobe",
        item_type: ItemType::Armour
    }],
    
};
