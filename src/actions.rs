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
    Manage,
    Equip,
    EquipItem,
}

#[derive(Clone)]
pub struct Action {
    pub class: ActionType,
    pub name: String,
}

impl Action {
    pub fn new(class: ActionType, name: String) -> Action {
        Action { class, name }
    }
}
