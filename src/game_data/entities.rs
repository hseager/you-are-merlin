use crate::{characters::enemy::Enemy, item::quest_item::QuestItem};

#[derive(Clone, Debug)]
pub struct Location {
    pub name: String,
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
    pub character: String,
    pub location_name: String,
    pub item: QuestItem,
}

#[derive(Clone, Debug)]
pub struct MainQuest {
    pub character: String,
    pub world_name: &'static str,
    pub boss_name: String,
}
