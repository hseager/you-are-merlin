use colored::{Color, Colorize};

#[derive(Clone)]
pub enum ActionType {
    Travel,
    Explore,
    Attack,
    MoveToLocation,
}

#[derive(Clone)]
pub struct Action {
    pub class: ActionType,
    pub name: &'static str,
    pub name_color: Color,
}

impl Action {
    pub fn display_name(&self) -> String {
        self.name.color(self.name_color).to_string()
    }

    pub fn new(class: ActionType, name: &'static str, name_color: Color) -> Action {
        Action {
            class,
            name,
            name_color,
        }
    }
}

pub fn get_visiting_actions() -> Vec<Action> {
    vec![
        Action::new(ActionType::Travel, "Travel", Color::Yellow),
        Action::new(ActionType::Explore, "Explore", Color::Blue),
    ]
}
pub fn get_exploring_actions() -> Vec<Action> {
    vec![Action::new(ActionType::Attack, "Attack", Color::Red)]
}
