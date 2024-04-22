use crate::{
    actions::{Action, ActionType},
    characters::player::Player,
    game_data::entities::{Encounter, MainQuest},
    game_state::GameState,
    text_format::TextFormatter,
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
            Action::new(ActionType::Continue, "Continue".text_green()),
            Action::new(ActionType::Run, "Run".text_cyan()),
        ]
    }

    fn handle_action(
        &mut self,
        action: Action,
        game_state: &mut GameState,
        _player: &mut Player,
    ) -> EventResponse {
        match action.class {
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
            ActionType::Run => EventResponse::new(Some(VisitEvent::build(game_state)), None),
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}
