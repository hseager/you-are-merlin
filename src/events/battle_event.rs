use colored::Colorize;

use crate::{
    actions::{Action, ActionType},
    game_data::entities::Battle,
    game_state::GameState,
};

use super::{event::Event, travel_event::TravelEvent, visit_event::VisitEvent};

pub struct BattleEvent {
    battle: Battle,
}

impl BattleEvent {
    pub fn new(battle: Battle) -> BattleEvent {
        BattleEvent { battle }
    }
}

impl Event for BattleEvent {
    fn prompt(&self) -> String {
        format!(
            "A wild {} appears! (life: {}, attack: {})\n{}",
            &self.battle.enemy.name,
            &self.battle.enemy.life,
            &self.battle.enemy.attack,
            &self.battle.enemy.description
        )
    }

    fn actions(&self) -> Vec<Action> {
        vec![
            Action::new(ActionType::Attack, "Attack".red()),
            Action::new(ActionType::Run, "Run".cyan()),
        ]
    }

    fn handle_action(
        &self,
        _search: &str,
        action_type: ActionType,
        game_state: &mut GameState,
    ) -> Box<dyn Event> {
        match action_type {
            ActionType::Attack => Box::new(TravelEvent::new(game_state.get_locations())),
            ActionType::Run => Box::new(VisitEvent::new(
                game_state.get_current_location().clone(),
                game_state.completed_locations.clone(),
            )),
            _ => panic!("Unhandled action when handling action."),
        }
    }
}
