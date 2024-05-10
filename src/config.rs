pub const PLAYER_LIFE: i16 = 100;
pub const PLAYER_ATTACK: u16 = 5;
pub const PLAYER_ATTACK_SPEED: u16 = 20;
pub const PLAYER_BLOCK: u16 = 0;
pub const PLAYER_CRIT_CHANCE: f32 = 2.0;
pub const PLAYER_CRIT_MULTI: f32 = 1.6;
pub const PLAYER_PARRY_CHANCE: f32 = 0.0;
pub const PLAYER_DODGE_CHANCE: f32 = 0.0;

// We set a base attack time in milliseconds so that we can have the attack speed
// stat as a number. Attack speed is calculated by (base_attack - (x * 10)). So
// 20 attack speed makes attacks 200ms quicker
pub const FIGHTER_BASE_ATTACK_SPEED: u16 = 4200;

pub const REST_HEAL_AMOUNT: i16 = 10;
pub const REST_INTERVAL_MILLIS: u32 = 2000;

// We have a fast battle interval as the fighters attack speed dictates when the
// event updates
pub const BATTLE_INTERVAL_MILLIS: u32 = 100;
pub const BATTLE_BASE_CHANCE_TO_FIND_ITEM: u16 = 75;
pub const BATTLE_CHANCE_TO_FIND_ITEM_DECAY: u16 = 8;

// life, attack, attack speed
pub const ENEMY_EASY_STATS: (i16, u16, u16) = (20, 7, 30);
pub const ENEMY_MEDIUM_STATS: (i16, u16, u16) = (25, 9, 50);
pub const ENEMY_HARD_STATS: (i16, u16, u16) = (28, 12, 80);
pub const ENEMY_BOSS_STATS: (i16, u16, u16) = (100, 20, 100);

// min, max
pub const ITEM_GEN_POWER: (u16, u16) = (2, 12);
pub const ITEM_GEN_ATTACK_SPEED: (u16, u16) = (50, 250);
pub const ITEM_GEN_CRIT_MULTI: (f32, f32) = (0.2, 2.5);
pub const ITEM_GEN_CRIT_CHANCE: (f32, f32) = (4.0, 20.0);

pub const ITEM_GEN_MAX_LIFE: (i16, i16) = (10, 50);
pub const ITEM_GEN_BLOCK: (u16, u16) = (1, 8);
pub const ITEM_GEN_PARRY_CHANCE: (f32, f32) = (10.0, 35.0);
pub const ITEM_GEN_DODGE_CHANCE: (f32, f32) = (5.0, 20.0);
