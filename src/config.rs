pub const PLAYER_LIFE: i16 = 100;
pub const PLAYER_ATTACK: u16 = 10;
pub const PLAYER_ATTACK_SPEED: u16 = 2000; // DPS: 5

pub const REST_HEAL_AMOUNT: i16 = 10;
pub const REST_INTERVAL_MILLIS: u32 = 2000;

pub const BATTLE_INTERVAL_MILLIS: u32 = 100;

// life, attack, attack speed (millisecs)
pub const ENEMY_EASY_STATS: (i16, u16, u16) = (20, 3, 1000); // DPS: 3
pub const ENEMY_MEDIUM_STATS: (i16, u16, u16) = (26, 5, 1200); // DPS: 6
pub const ENEMY_HARD_STATS: (i16, u16, u16) = (38, 14, 1800); // DPS: 7.7
pub const ENEMY_BOSS_STATS: (i16, u16, u16) = (150, 8, 1000); // DPS: 8

// min, max
pub const ITEM_GEN_ATTACK: (u16, u16) = (1, 4);
pub const ITEM_GEN_LIFE: (i16, i16) = (5, 10);
