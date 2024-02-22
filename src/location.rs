use colored::{Color, Colorize};

#[derive(Copy, Clone)]
pub struct Location {
    pub name: &'static str,
    pub name_color: Color,
    pub description: &'static str,
}

impl Location {
    pub fn display_name(&self) -> String {
        self.name.color(self.name_color).to_string()
    }
}
