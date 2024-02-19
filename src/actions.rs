use colored::{Color, Colorize};

pub enum Action {
    Travel,
    Explore,
    Attack,
    CastSpell,
}

pub struct ActionItem {
    pub class: Action,
    pub label: &'static str,
    pub label_color: Color,
}

impl ActionItem {
    pub fn new(class: Action, label: &'static str, label_color: Color) -> Self {
        Self {
            class,
            label,
            label_color
        }
    }

    pub fn display_label(&self) -> String {
        self.label.color(self.label_color).to_string()
    }
}