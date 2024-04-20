use rand::Rng;

pub trait Fighter {
    fn name(&self) -> String;
    fn life(&self) -> &i16;
    fn is_alive(&self) -> bool {
        self.life() > &0
    }
    fn attack(&self, target: &mut dyn Fighter) -> String;
    fn take_damage(&mut self, damage: u16);
}

// Select random damage from -2 to +2 of current attack stat
pub fn calculate_damage(attack: u16) -> u16 {
    let damage_range = 2;
    rand::thread_rng().gen_range(attack - damage_range..=attack + damage_range)
}
