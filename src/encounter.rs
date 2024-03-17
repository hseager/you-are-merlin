use colored::ColoredString;

use crate::characters::Enemy;

#[derive(Clone, Debug)]
pub enum Encounter {
    BossFight(Battle),
    Battle(Battle),
    Quest(Quest),
}

#[derive(Clone, Debug)]
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
    pub character: ColoredString,
    pub item: ColoredString,
}

#[derive(Clone, Debug)]
pub struct MainQuest {
    pub character: ColoredString,
    pub boss_name: ColoredString,
}
