use colored::{Color, Colorize};

pub enum Action {
    Travel,
    Explore,
    UseItem,
    Attack,
    CastSpell,
}

pub struct ActionItem {
    pub action: Action,
    pub label: &'static str,
    pub label_color: Color,
}

impl ActionItem {
    pub fn new(action: Action, label: &'static str, label_color: Color) -> Self {
        Self {
            action,
            label,
            label_color
        }
    }

    pub fn display_label(&self) -> String {
        self.label.color(self.label_color).to_string()
    }
}