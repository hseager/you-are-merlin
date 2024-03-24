use colored::{ColoredString, Colorize};
use rand::{seq::SliceRandom, thread_rng, Rng};

pub struct Item {
    pub name: ColoredString,
    pub max_life: i16,
    pub attack: u16,
}

pub fn get_encounter_reward(items: &mut Vec<&str>) -> Item {
    let mut rng = thread_rng();

    let item_name = items.choose_mut(&mut rng).unwrap().to_owned();
    items.retain(|c| *c != item_name);

    let (max_life, attack) = create_item_stats();

    Item {
        name: item_name.bold(),
        attack,
        max_life,
    }
}

pub fn create_item_stats() -> (i16, u16) {
    let attack: u16 = rand::thread_rng().gen_range(1..4);
    let max_life = rand::thread_rng().gen_range(5..10);

    (max_life, attack)
}
