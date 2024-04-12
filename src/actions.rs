use colored::ColoredString;

#[derive(Clone)]
pub enum ActionType {
    Travel,
    Explore,
    Attack,
    Run,
    MoveToLocation,
    Rest,
    Accept,
    Continue,
    GiveItem,
    Open,
}

#[derive(Clone)]
pub struct Action {
    pub class: ActionType,
    pub name: ColoredString,
}

impl Action {
    pub fn new(class: ActionType, name: ColoredString) -> Action {
        Action { class, name }
    }
}
