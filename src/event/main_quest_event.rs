use colored::Colorize;

use crate::{
    actions::{Action, ActionType},
    characters::player::Player,
    game_data::entities::{Encounter, MainQuest},
    game_state::GameState,
};

use super::{
    battle_event::BattleEvent, event_loop::EventLoop, event_response::EventResponse,
    visit_event::VisitEvent, Event,
};

pub struct MainQuestEvent {
    quest: MainQuest,
}

impl MainQuestEvent {
    pub fn new(quest: MainQuest) -> MainQuestEvent {
        MainQuestEvent { quest }
    }
}

impl Event for MainQuestEvent {
    fn prompt(&self) -> Option<String> {
        let MainQuest {
            character,
            world_name,
            boss_name,
        } = &self.quest;

        Some(format!(
            "You find a calm area. {} wants to ask you something.\n\"{} is in great danger... {} seeks the destruction of this world... They must be stopped...\"",
            character, world_name, boss_name
        ))
    }

    fn actions(&self) -> Vec<Action> {
        vec![
            Action::new(ActionType::Continue, "Continue".green()),
            Action::new(ActionType::Run, "Run".cyan()),
        ]
    }

    fn handle_action(
        &mut self,
        _search: &str,
        action_type: ActionType,
        game_state: &mut GameState,
        _player: &mut Player,
    ) -> EventResponse {
        match action_type {
            ActionType::Continue => match game_state.go_to_next_encounter() {
                Some(encounter) => match encounter {
                    Encounter::Battle(battle) => {
                        let next_event = Box::new(BattleEvent::new(battle.clone()));
                        EventResponse::new(Some(next_event), None)
                    }
                    _ => panic!("Unhandled encounter after accepting main quest."),
                },
                None => panic!("Shouldn't be the end of encounters after accepting main quest..."),
            },
            ActionType::Run => {
                let next_event = Box::new(VisitEvent::new(
                    game_state.get_current_location().clone(),
                    game_state.completed_locations.clone(),
                ));
                EventResponse::new(Some(next_event), None)
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}