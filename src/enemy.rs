#[derive(Clone, Copy)]
pub struct Enemy {
    pub name: &'static str,
    pub life: i16,
    pub attack: i16,
}
