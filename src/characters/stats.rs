#[derive(Clone, Debug)]
pub struct Stats {
    pub life: i16,
    pub max_life: i16,
    pub power: u16,
    pub attack_speed: u16,
    pub crit_multiplier: f32,
    pub crit_chance: f32,
    pub block: u16,
    pub parry: f32,
    pub dodge: f32,
}

impl Stats {
    pub fn new(life: i16, power: u16, attack_speed: u16) -> Stats {
        Stats {
            life,
            max_life: life,
            power,
            attack_speed,
            crit_multiplier: 0.0,
            crit_chance: 0.0,
            block: 0,
            parry: 0.0,
            dodge: 0.0,
        }
    }
}
