pub const PLAYER_LIFE: i16 = 100;
pub const PLAYER_ATTACK: u16 = 10;
pub const PLAYER_ATTACK_SPEED: u16 = 4000;

pub const REST_HEAL_AMOUNT: i16 = 10;
pub const REST_INTERVAL_MILLIS: u32 = 2000;

pub const BATTLE_INTERVAL_MILLIS: u32 = 100;

// life, attack, attack speed (millisecs)
pub const ENEMY_EASY_STATS: (i16, u16, u16) = (20, 7, 3800);
pub const ENEMY_MEDIUM_STATS: (i16, u16, u16) = (25, 9, 3500);
pub const ENEMY_HARD_STATS: (i16, u16, u16) = (28, 12, 3200);
pub const ENEMY_BOSS_STATS: (i16, u16, u16) = (100, 20, 3000);

// min, max
pub const ITEM_GEN_ATTACK: (u16, u16) = (1, 4);
pub const ITEM_GEN_LIFE: (i16, i16) = (5, 10);
