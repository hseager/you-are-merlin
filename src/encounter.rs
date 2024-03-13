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
pub enum Quest {
    MainQuest(MainQuest),
    SideQuest(SideQuest),
}

#[derive(Clone, Debug)]
pub struct SideQuest {
    pub character: &'static str,
    pub item: &'static str,
}

#[derive(Clone, Debug)]
pub struct MainQuest {
    pub character: &'static str,
}
