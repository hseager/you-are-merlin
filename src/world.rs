use crate::{encounter::Encounter, location::Location};

pub struct World {
    pub name: &'static str,
    pub current_location: usize,
    pub locations: Vec<Location>,
}

impl World {
    pub fn get_current_location(&self) -> Option<&Location> {
        self.locations.get(self.current_location)
    }

    // TODO clean this up
    pub fn get_current_encounter(&self) -> &Encounter {
        let location = self.get_current_location().unwrap();

        location
            .encounters
            .get(location.current_encounter)
            .expect("Couldn't get encounter")
    }
}
