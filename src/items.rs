use colored::{ColoredString, Colorize};
use rand::{seq::SliceRandom, thread_rng};

pub struct Item {
    pub name: ColoredString,
    pub attack: u16,
    pub life: i16,
}

pub fn get_encounter_reward(items: &mut Vec<&str>) -> Item {
    let mut rng = thread_rng();

    let item_name = items.choose_mut(&mut rng).unwrap().to_owned();
    items.retain(|c| *c != item_name);

    Item {
        name: item_name.bold(),
        attack: 2,
        life: 5,
    }
}
