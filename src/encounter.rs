use crate::enemy::Enemy;

// #[derive(Clone, Copy)]
// pub enum EncounterType {
//     Battle,
//     Quest,
// }

// #[derive(Clone, Copy)]
// pub struct Battle {
//     pub class: EncounterType,
//     pub enemy: Enemy,
//     // pub reward: u8, TODO Rewards
// }

pub enum Encounter {
    Battle(Battle),
    Quest(Quest),
}

pub enum Battle {
    Enemy(Enemy),
}

pub enum Quest {
    Character(String),
}
