#[derive(Clone, Copy, Debug)]
pub struct Enemy {
    pub name: &'static str,
    pub life: i16,
    pub attack: i16,
}
