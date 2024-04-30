pub trait DisplayStats {
    fn display_attacks_per_second(&self) -> u16;
}

#[derive(Clone, Debug)]
pub struct Stats {
    pub max_life: i16,
    pub life: i16,
    pub power: u16,
    pub attack_speed: u16,
}

// TODO need to work out how to calculate attack speed
impl DisplayStats for Stats {
    fn display_attacks_per_second(&self) -> u16 {
        self.attack_speed / 1000
    }
}
