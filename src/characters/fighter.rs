use rand::Rng;

pub trait Fighter {
    fn name(&self) -> String;
    fn life(&self) -> &i16;
    fn is_alive(&self) -> bool {
        self.life() > &0
    }
    fn attack(&self, target: &mut dyn Fighter) -> String;
    fn take_damage(&mut self, damage: u16);
    fn attack_speed_as_milliseconds(&self) -> u16;
    fn can_attack(&self, current_time: i32, last_attack_time: i32) -> bool {
        current_time - last_attack_time >= self.attack_speed_as_milliseconds() as i32
    }

    fn power(&self) -> u16;
    fn attack_speed(&self) -> u16;
}

// Select random damage from -2 to +2 of current attack stat
pub fn calculate_damage(power: u16) -> u16 {
    let damage_range = 2;
    rand::thread_rng().gen_range(power - damage_range..=power + damage_range)
}
