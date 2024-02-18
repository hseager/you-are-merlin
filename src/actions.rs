use colored::Colorize;
use std::str::FromStr;

pub enum Action {
    Travel,
    Explore,
    UseItem,
    Attack,
    CastSpell,
}

impl Action {
    pub fn to_string(&self) -> String {
        match self {
            Action::Travel => "Travel".blue().to_string(),
            Action::Explore => "Explore".yellow().to_string(),
            Action::UseItem => "Use item".purple().to_string(),
            Action::Attack => "Attack".red().to_string(),
            Action::CastSpell => "Cast spell".cyan().to_string(),
        }
    }
}

impl FromStr for Action {
    type Err = ();

    fn from_str(input: &str) -> Result<Action, Self::Err> {
        match input {
            "Travel" => Ok(Action::Travel),
            "Explore" => Ok(Action::Explore),
            "Use item" => Ok(Action::UseItem),
            "Attack" => Ok(Action::Attack),
            "Cast spell" => Ok(Action::CastSpell),
            _ => Err(()),
        }
    }
}

pub fn get_map_actions() -> Vec<Action> {
    vec![Action::Travel, Action::Explore]
}
