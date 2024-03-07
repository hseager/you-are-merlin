use crate::enemy::Enemy;

#[derive(Clone)]
pub enum Encounter {
    Battle(Battle),
    Quest(Quest),
}

#[derive(Clone, Copy)]
pub struct Battle {
    pub enemy: Enemy,
}

#[derive(Clone)]
pub struct Quest {
    pub character: &'static str,
}
