use colored::ColoredString;

#[derive(Clone, Debug)]
pub struct Enemy {
    pub name: ColoredString,
    pub description: &'static str,
    pub life: i16,
    pub attack: i16,
}
