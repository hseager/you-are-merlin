#[derive(Clone, Copy)]
pub enum EncounterType {
    Battle,
}

#[derive(Clone, Copy)]
pub struct Encounter {
    pub class: EncounterType,
    pub enemy: Enemy,
    pub reward: u8,
}

#[derive(Clone, Copy)]
pub struct Enemy {
    pub name: &'static str,
    pub life: i16,
    pub attack: i16,
}
