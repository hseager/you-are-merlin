use colored::{Color, Colorize};

use crate::encounter::Battle;

#[derive(Clone)]
pub struct Location {
    pub name: &'static str,
    pub name_color: Color,
    pub description: &'static str,
    pub current_encounter: usize,
    pub encounters: Vec<Battle>,
}

impl Location {
    pub fn display_name(&self) -> String {
        self.name.color(self.name_color).to_string()
    }

    pub fn reset_encounters(&mut self) -> () {
        self.current_encounter = 0;
        for encounter in &mut self.encounters {
            encounter.enemy.life = 20; // TODO reset to original enemy life somehow
        }
    }
}
