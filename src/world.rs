use crate::location::Location;

pub struct World {
    pub name: &'static str,
    pub current_location: usize,
    pub locations: Vec<Location>,
}

impl World {
    pub fn get_current_location(&self) -> &Location {
        &self.locations.get(self.current_location).unwrap()
    }
}
