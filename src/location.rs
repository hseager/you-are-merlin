use colored::{Color, Colorize};

use crate::encounter::Encounter;

#[derive(Clone)]
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
}
