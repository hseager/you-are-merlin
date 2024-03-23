use colored::ColoredString;

use crate::characters::Enemy;

// todo add locaton type like dungeons and safe zones
// Boss location, mini bosses for dungeons

#[derive(Debug)]
pub struct Location {
    pub name: ColoredString,
    pub description: &'static str,
    pub encounters: Vec<Encounter>,
    pub class: LocationType,
}

#[derive(Debug, Clone, Copy)]
pub enum LocationType {
    SafeZone,
    Dungeon,
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

#[derive(Clone, Copy)]
pub enum EnemyDifficulty {
    Easy,
    Normal,
    Hard,
    Boss,
}
