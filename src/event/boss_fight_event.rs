use crate::{
    actions::{Action, ActionType},
    characters::{enemy::Enemy, player::Player},
    game_data::entities::Battle,
    game_state::GameState,
    text_format::TextFormatter,
};

use super::{
    event_loop::{
        battle_event_loop::{BattleEventLoop, Turn},
        EventLoop,
    },
    Event, EventResponse,
};

pub struct BossFightEvent {
    enemy: Enemy,
    event_loop: BattleEventLoop,
}

impl BossFightEvent {
    pub fn new(battle: Battle) -> BossFightEvent {
        let event_loop = BattleEventLoop::new(battle.enemy.clone(), Turn::Player);

        BossFightEvent {
            enemy: battle.enemy,
            event_loop,
        }
    }
}

impl Event for BossFightEvent {
    fn prompt(&self) -> Option<String> {
        Some(format!(
            "A great danger approaches...\n\
        {} (life: {}, attack: {})\n\
        {}",
            self.enemy.name, self.enemy.life, self.enemy.attack, self.enemy.description
        ))
    }

    fn actions(&self) -> Vec<Action> {
        vec![Action::new(ActionType::Attack, "Attack".text_red())]
    }

    fn handle_action(
        &mut self,
        _search: &str,
        action_type: ActionType,
        _game_state: &mut GameState,
        _player: &mut Player,
    ) -> EventResponse {
        match action_type {
            ActionType::Attack => {
                self.event_loop.is_active = true;
                EventResponse::new(None, None)
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        Some(&mut self.event_loop)
    }
}
