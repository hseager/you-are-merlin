use colored::{ColoredString, Colorize};

use crate::game_data::entities::{Location, LocationType, Quest, SideQuest};

#[derive(Clone)]
pub enum ActionType {
    Travel,
    Explore,
    Attack,
    Run,
    MoveToLocation,
    Rest,
    Accept,
    Continue,
}

#[derive(Clone)]
pub struct Action {
    pub class: ActionType,
    pub name: ColoredString,
}

impl Action {
    pub fn new(class: ActionType, name: ColoredString) -> Action {
        Action { class, name }
    }
}

pub fn get_visiting_actions(location: &Location) -> Vec<Action> {
    let mut actions = vec![
        Action::new(ActionType::Travel, "Travel".yellow()),
        Action::new(ActionType::Explore, "Explore".blue()),
    ];

    if let LocationType::SafeZone = location.class {
        actions.push(Action::new(ActionType::Rest, "Rest".green()));
    }

    actions
}
pub fn get_battle_actions() -> Vec<Action> {
    vec![
        Action::new(ActionType::Attack, "Attack".red()),
        Action::new(ActionType::Run, "Run".cyan()),
    ]
}

// TODO need to check accepted quests here and be able to change options based on inventory etc
pub fn get_quest_actions(quest: &Quest, accepted_quests: &Vec<&SideQuest>) -> Vec<Action> {
    match quest {
        Quest::MainQuest(_) => {
            vec![
                Action::new(ActionType::Continue, "Continue".green()),
                Action::new(ActionType::Run, "Run".cyan()),
            ]
        },
        Quest::SideQuest(quest) => {
            let accepted_quest = accepted_quests.iter().any(|q| q.character == quest.character);

            if accepted_quest {
                vec![
                    Action::new(ActionType::Run, "Continue".green())
                ]
            } else {
                vec![
                    Action::new(ActionType::Accept, "Accept".green()),
                    Action::new(ActionType::Run, "Decline".red()),
                ]
            }
        }
    }

}

pub fn get_locations_as_actions(locations: &[Location]) -> Vec<Action> {
    locations
        .iter()
        .map(|location| Action::new(ActionType::MoveToLocation, location.name.to_owned()))
        .collect()
}

pub fn get_treasure_actions() -> Vec<Action> {
    vec![Action::new(ActionType::Travel, "Travel".yellow())]
}
