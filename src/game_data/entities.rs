use colored::ColoredString;

use crate::characters::enemy::Enemy;

#[derive(Clone, Debug)]
pub struct Location {
    pub name: ColoredString,
    pub description: &'static str,
    pub encounters: Vec<Encounter>,
    pub class: LocationType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LocationType {
    SafeZone,
    Dungeon(&'static str),
    BossDungeon,
}

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
    pub location_name: ColoredString,
    pub item: ColoredString,
}

#[derive(Clone, Debug)]
pub struct MainQuest {
    pub character: ColoredString,
    pub world_name: &'static str,
    pub boss_name: ColoredString,
}

#[derive(Clone, Copy, Debug)]
pub enum EnemyDifficulty {
    Easy,
    Normal,
    Hard,
    Boss,
}
