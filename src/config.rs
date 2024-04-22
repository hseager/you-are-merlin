pub const PLAYER_LIFE: i16 = 100;
pub const PLAYER_ATTACK: u16 = 5;
pub const PLAYER_ATTACK_SPEED: u64 = 2000;

pub const REST_HEAL_AMOUNT: i16 = 10;
pub const REST_INTERVAL_MILLIS: u32 = 2000;

pub const BATTLE_INTERVAL_MILLIS: u32 = 100;

// life, attack, attack speed
pub const ENEMY_EASY_STATS: (i16, u16, u64) = (20, 3, 1600);
pub const ENEMY_MEDIUM_STATS: (i16, u16, u64) = (26, 5, 1800);
pub const ENEMY_HARD_STATS: (i16, u16, u64) = (38, 6, 1500);
pub const ENEMY_BOSS_STATS: (i16, u16, u64) = (150, 10, 1000);

// min, max
pub const ITEM_GEN_ATTACK: (u16, u16) = (1, 4);
pub const ITEM_GEN_LIFE: (i16, i16) = (5, 10);
