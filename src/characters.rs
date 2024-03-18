use colored::ColoredString;
use rand::Rng;

pub trait Fighter {
    fn name(&self) -> &ColoredString;
    fn life(&self) -> &i16;
    fn is_alive(&self) -> bool {
        self.life() > &0
    }
    fn attack(&self, target: &mut dyn Fighter) -> ();
    fn take_damage(&mut self, damage: i16) -> ();
}

#[derive(Clone, Debug)]
pub struct Enemy {
    pub name: ColoredString,
    pub description: &'static str,
    pub life: i16,
    pub attack: i16,
}

impl Fighter for Enemy {
    fn name(&self) -> &ColoredString {
        &self.name
    }

    fn life(&self) -> &i16 {
        &self.life
    }

    fn attack(&self, target: &mut dyn Fighter) -> () {
        let damage = calculate_damage(self.attack);
        target.take_damage(damage);

        println!(
            "{} attacks you for {} damage. (Your life: {})",
            &self.name(),
            damage,
            &self.life()
        );
    }

    fn take_damage(&mut self, damage: i16) -> () {
        self.life -= damage;
    }
}

pub struct Player {
    pub name: ColoredString,
    pub life: i16,
    pub attack: i16,
}

impl Fighter for Player {
    fn name(&self) -> &ColoredString {
        &self.name
    }

    fn life(&self) -> &i16 {
        &self.life
    }

    fn attack(&self, target: &mut dyn Fighter) -> () {
        let damage = calculate_damage(self.attack);
        target.take_damage(damage);

        println!(
            "You attack {} for {} damage. (Enemy life: {})",
            &target.name(),
            damage,
            &self.life()
        );
    }

    fn take_damage(&mut self, damage: i16) -> () {
        self.life -= damage;
    }
}

// Select random damage from -2 to +2 of current attack stat
fn calculate_damage(damage: i16) -> i16 {
    let range = rand::thread_rng().gen_range(0..4);

    match range {
        0 => damage - 2,
        1 => damage - 1,
        2 => damage,
        3 => damage + 1,
        4 => damage + 2,
        _ => damage,
    }
}
