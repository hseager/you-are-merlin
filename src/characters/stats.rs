#[derive(Clone, Debug)]
pub struct Stats {
    pub max_life: i16,
    pub life: i16,
    pub power: u16,
    pub attack_speed: u16,
    pub block: u16,
    pub crit_multiplier: f32,
    pub crit_chance: f32,
    pub parry: f32,
    pub dodge: f32,
}
