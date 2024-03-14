use crate::enemy::Enemy;

#[derive(Clone, Debug)]
pub enum Encounter {
    Combat(Combat),
    Quest(Quest),
}

// TODO change Battle to enum and have BossBattles and EnemyFight

#[derive(Clone, Copy, Debug)]
pub struct Battle {
    pub enemy: Enemy,
}

#[derive(Clone, Copy, Debug)]
pub enum Combat {
    BossFight(Battle),
    Skirmish(Battle),
}

#[derive(Clone, Debug)]
pub enum Quest {
    MainQuest(MainQuest),
    SideQuest(SideQuest),
}

#[derive(Clone, Debug)]
pub struct SideQuest {
    pub character: String,
    pub item: String,
}

#[derive(Clone, Debug)]
pub struct MainQuest {
    pub character: String,
    pub boss_name: String,
}
