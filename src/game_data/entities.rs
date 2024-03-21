use colored::ColoredString;

use crate::characters::Enemy;

#[derive(Debug)]
pub struct Location {
    pub name: ColoredString,
    pub description: &'static str,
    pub encounters: Vec<Encounter>,
}

#[derive(Debug)]
pub enum Encounter {
    BossFight(Battle),
    Battle(Battle),
    Quest(Quest),
}

#[derive(Debug)]
pub struct Battle {
    pub enemy: Enemy,
}

#[derive(Debug)]
pub enum Quest {
    MainQuest(MainQuest),
    SideQuest(SideQuest),
}

#[derive(Debug)]
pub struct SideQuest {
    pub character: ColoredString,
    pub item: ColoredString,
}

#[derive(Debug)]
pub struct MainQuest {
    pub character: ColoredString,
    pub world_name: &'static str,
    pub boss_name: ColoredString,
}
