use crate::enemy::Enemy;

#[derive(Clone, Copy)]
pub enum EncounterType {
    Battle,
}

#[derive(Clone, Copy)]
pub struct Encounter {
    pub class: EncounterType,
    pub enemy: Enemy,
    // pub reward: u8, TODO Rewards
}
