use colored::ColoredString;

use crate::characters::Enemy;

#[derive(Debug)]
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
    pub location_name: ColoredString,
    pub item: ColoredString,
}

impl SideQuest {
    pub fn is_accepted(&self, accepted_quests: &[&SideQuest]) -> bool {
        accepted_quests
            .iter()
            .any(|q| q.character == self.character)
    }
}

#[derive(Debug)]
pub struct MainQuest {
    pub character: ColoredString,
    pub world_name: &'static str,
    pub boss_name: ColoredString,
}

#[derive(Clone, Copy)]
pub enum EnemyDifficulty {
    Easy,
    Normal,
    Hard,
    Boss,
}
