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

// life, attack, attack speed, crit multi, crit chance
pub const ENEMY_EASY_STATS: (i16, u16, u16, f32, f32) = (20, 7, 30, 1.2, 1.0);
pub const ENEMY_MEDIUM_STATS: (i16, u16, u16, f32, f32) = (25, 9, 50, 1.2, 1.0);
pub const ENEMY_HARD_STATS: (i16, u16, u16, f32, f32) = (28, 12, 80, 1.2, 1.0);
pub const ENEMY_BOSS_STATS: (i16, u16, u16, f32, f32) = (100, 15, 100, 1.2, 1.0);

// We add random defensive stats for each enemy type
pub const ENEMY_EASY_STAT_COUNT: usize = 1;
pub const ENEMY_MEDIUM_STAT_COUNT: usize = 2;
pub const ENEMY_HARD_STAT_COUNT: usize = 3;
pub const ENEMY_BOSS_STAT_COUNT: usize = 4;

// For each enemy stat we get a random range, min - max
// TODO do enemy offensive stats
pub const ENEMY_STAT_POWER_RANGE: (u16, u16) = (2, 4);
pub const ENEMY_STAT_ATTACK_SPEED_RANGE: (u16, u16) = (20, 100);
pub const ENEMY_STAT_CRIT_MULTI_RANGE: (f32, f32) = (0.2, 0.5);
pub const ENEMY_STAT_CRIT_CHANCE_RANGE: (f32, f32) = (4.0, 10.0);

pub const ENEMY_STAT_MAX_LIFE_RANGE: (i16, i16) = (5, 30);
pub const ENEMY_STAT_BLOCK_RANGE: (u16, u16) = (2, 4);
pub const ENEMY_STAT_PARRY_CHANCE: (f32, f32) = (10.0, 20.0);
pub const ENEMY_STAT_DODGE_CHANCE: (f32, f32) = (4.0, 10.0);

// min, max
pub const ITEM_GEN_POWER: (u16, u16) = (2, 12);
pub const ITEM_GEN_ATTACK_SPEED: (u16, u16) = (50, 175);
pub const ITEM_GEN_CRIT_MULTI: (f32, f32) = (0.2, 1.8);
pub const ITEM_GEN_CRIT_CHANCE: (f32, f32) = (4.0, 20.0);

pub const ITEM_GEN_MAX_LIFE: (i16, i16) = (10, 50);
pub const ITEM_GEN_BLOCK: (u16, u16) = (1, 8);
pub const ITEM_GEN_PARRY_CHANCE: (f32, f32) = (10.0, 35.0);
pub const ITEM_GEN_DODGE_CHANCE: (f32, f32) = (5.0, 20.0);

// % chance / common, rare, epic, legendary
pub const REWARD_CHANCE_BATTLE: (f32, f32, f32, f32) = (60.0, 30.0, 10.0, 0.0);
pub const REWARD_CHANCE_CHEST: (f32, f32, f32, f32) = (0.0, 70.0, 20.0, 10.0);
pub const REWARD_CHANCE_QUEST: (f32, f32, f32, f32) = (0.0, 45.0, 35.0, 20.0);
