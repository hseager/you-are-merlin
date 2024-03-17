use colored::ColoredString;

use crate::encounter::Encounter;

// TODO rather than storing name_color, just store ColoredString

#[derive(Clone, Debug)]
pub struct Location {
    pub name: ColoredString,
    pub description: &'static str,
    pub current_encounter: usize,
    pub encounters: Vec<Encounter>,
}

impl Location {
    pub fn reset_encounters(&mut self) -> () {
        self.current_encounter = 0;
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
