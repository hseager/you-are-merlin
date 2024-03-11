use crate::enemy::Enemy;

#[derive(Clone, Debug)]
pub enum Encounter {
    Battle(Battle),
    Quest(Quest),
}

#[derive(Clone, Copy, Debug)]
pub struct Battle {
    pub enemy: Enemy,
}

#[derive(Clone, Debug)]
pub struct Quest {
    pub character: &'static str,
    pub item: &'static str,
}
