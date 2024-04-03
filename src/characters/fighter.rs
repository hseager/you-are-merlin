use colored::ColoredString;

pub trait Fighter {
    fn name(&self) -> ColoredString;
    fn life(&self) -> &i16;
    fn is_alive(&self) -> bool {
        self.life() > &0
    }
    fn attack(&self, target: &mut dyn Fighter) -> String;
    fn take_damage(&mut self, damage: u16);
}
