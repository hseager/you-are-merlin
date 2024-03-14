use colored::{Color, Colorize};

use crate::encounter::Encounter;

#[derive(Clone, Debug)]
pub struct Location {
    pub name: &'static str,
    pub name_color: Color,
    pub description: &'static str,
    pub current_encounter: usize,
    pub encounters: Vec<Encounter>,
}

impl Location {
    pub fn display_name(&self) -> String {
        self.name.color(self.name_color).to_string()
    }

    pub fn reset_encounters(&mut self) -> () {
        self.current_encounter = 0;
    }

    pub fn get_current_encounter(&self) -> &Encounter {
        self.encounters
            .get(self.current_encounter)
            .expect("Couldn't get encounter")
    }

    pub fn go_to_next_encounter(&mut self) -> () {
        // TODO what happens at the end of all encounters?
        let next_encounter = self.current_encounter + 1;

        if next_encounter < self.encounters.len() {
            self.current_encounter = next_encounter;
        } else {
            self.reset_encounters();
        }
    }
}
