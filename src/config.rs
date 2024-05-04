pub const PLAYER_LIFE: i16 = 100;
pub const PLAYER_ATTACK: u16 = 5;
pub const PLAYER_ATTACK_SPEED: u16 = 20;
pub const PLAYER_BLOCK: u16 = 0;
pub const PLAYER_CRIT_CHANCE: f32 = 1.0;
pub const PLAYER_CRIT_MULTI: f32 = 1.2;
pub const PLAYER_PARRY_CHANCE: f32 = 0.0;
pub const PLAYER_DODGE_CHANCE: f32 = 0.0;

// We set a base attack time in milliseconds so that we can have the attack speed
// stat as a number. Attack speed is calculated by (base_attack - (x * 10)). So
// 20 attack speed makes attacks 200ms quicker
pub const FIGHTER_BASE_ATTACK_SPEED: u16 = 4200;

pub const REST_HEAL_AMOUNT: i16 = 10;
pub const REST_INTERVAL_MILLIS: u32 = 2000;

pub const BATTLE_INTERVAL_MILLIS: u32 = 100;

// life, attack, attack speed
pub const ENEMY_EASY_STATS: (i16, u16, u16) = (20, 7, 30);
pub const ENEMY_MEDIUM_STATS: (i16, u16, u16) = (25, 9, 50);
pub const ENEMY_HARD_STATS: (i16, u16, u16) = (28, 12, 80);
pub const ENEMY_BOSS_STATS: (i16, u16, u16) = (100, 20, 100);

// min, max
pub const ITEM_GEN_POWER: (u16, u16) = (4, 10);
pub const ITEM_GEN_ATTACK_SPEED: (u16, u16) = (12, 30);
pub const ITEM_GEN_CRIT_MULTI: (f32, f32) = (0.4, 2.0);
pub const ITEM_GEN_CRIT_CHANCE: (f32, f32) = (4.0, 6.0);

pub const ITEM_GEN_MAX_LIFE: (i16, i16) = (10, 20);
pub const ITEM_GEN_BLOCK: (u16, u16) = (1, 10);
pub const ITEM_GEN_PARRY_CHANCE: (f32, f32) = (2.0, 8.0);
pub const ITEM_GEN_DODGE_CHANCE: (f32, f32) = (2.0, 6.0);
