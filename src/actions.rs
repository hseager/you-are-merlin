use colored::{Color, Colorize};

pub enum ActionType {
    Travel,
    Explore,
    Attack,
    CastSpell,
}

pub struct Action {
    pub class: ActionType,
    pub name: &'static str,
    pub name_color: Color,
}

impl Action {
    pub fn display_name(&self) -> String {
        self.name.color(self.name_color).to_string()
    }
}
