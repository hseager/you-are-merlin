use crate::{
    actions::{Action, ActionType},
    characters::{enemy::Enemy, player::Player},
    game_data::entities::Battle,
    game_state::GameState,
    text_format::TextFormatter,
};

use super::{
    event_loop::{battle_event_loop::BattleEventLoop, EventLoop},
    visit_event::VisitEvent,
    Event, EventResponse,
};

pub struct BattleEvent {
    enemy: Enemy,
    event_loop: BattleEventLoop,
}

impl BattleEvent {
    pub fn new(battle: Battle) -> BattleEvent {
        let event_loop = BattleEventLoop::new(battle.enemy.clone());

        BattleEvent {
            enemy: battle.enemy,
            event_loop,
        }
    }
}

impl Event for BattleEvent {
    fn prompt(&self) -> Option<String> {
        println!("Stats: {:?}", &self.enemy.stats);
        Some(format!(
            "A {} appears! {}\n{}",
            &self.enemy.name,
            &self.enemy.description,
            &self.enemy.difficulty.description()
        ))
    }

    fn actions(&self) -> Vec<Action> {
        vec![
            Action::new(ActionType::Attack, "Attack".text_red()),
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
            ActionType::Attack => {
                self.event_loop.start();
                EventResponse::new(None, None)
            }
            ActionType::Run => {
                game_state.reset_encounters();

                EventResponse::new(Some(VisitEvent::build(game_state)), None)
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        Some(&mut self.event_loop)
    }
}
