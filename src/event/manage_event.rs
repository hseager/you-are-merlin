use crate::{
    actions::{Action, ActionType},
    characters::{player::Player, stats::Stats},
    game_state::GameState,
    item::{Equipment, Item, ItemType},
    text_format::TextFormatter,
};

use super::{event_loop::EventLoop, event_response::EventResponse, visit_event::VisitEvent, Event};

enum ManageState {
    Status,
    Equip,
}

pub struct ManageEvent {
    stats: Stats,
    equipment: Equipment,
    inventory: Vec<Box<dyn Item>>,
    state: ManageState,
}

impl ManageEvent {
    pub fn new(stats: Stats, equipment: Equipment, inventory: Vec<Box<dyn Item>>) -> ManageEvent {
        ManageEvent {
            stats,
            equipment,
            inventory,
            state: ManageState::Status,
        }
    }

    fn get_status_prompt(&self) -> Option<String> {
        let stats = format!(
            "You have {} life, {} power and {} attack speed.",
            self.stats.life, self.stats.power, self.stats.attack_speed
        );

        let mut equipment = String::from("Your equiped items:\n");

        if let Some(weapon) = &self.equipment.weapon {
            equipment.push_str(&format!("Weapon: {}\n", weapon.display_info()));
        } else {
            equipment.push_str("Weapon: None\n");
        }

        if let Some(armour) = &self.equipment.armour {
            equipment.push_str(&format!("Armor: {}\n", armour.display_info()));
        } else {
            equipment.push_str("Armor: None\n");
        }

        if let Some(artifact) = &self.equipment.artifact {
            equipment.push_str(&format!("Artifact: {}\n", artifact.display_info()));
        } else {
            equipment.push_str("Artifact: None\n");
        }

        let mut inventory = String::new();
        if self.inventory.is_empty() {
            inventory.push_str("You have no items in your inventory.")
        } else {
            inventory.push_str("You have these items in your inventory:");
            for item in &self.inventory {
                inventory = format!("{}\n{}", inventory, item.display_info());
            }
        }

        let response = format!("{}\n\n{}\n{}", stats, equipment, inventory);

        Some(response)
    }

    fn get_inventory_as_actions(&self) -> Vec<Action> {
        self.inventory
            .iter()
            .filter(|i| i.item_type() != ItemType::QuestItem)
            .map(|i| Action::new(ActionType::EquipItem, i.display_name()))
            .collect()
    }
}

impl Event for ManageEvent {
    fn prompt(&self) -> Option<String> {
        match self.state {
            ManageState::Equip => Some(String::from("What would you like to equip?")),
            ManageState::Status => self.get_status_prompt(),
        }
    }
    fn actions(&self) -> Vec<Action> {
        match self.state {
            ManageState::Equip => self.get_inventory_as_actions(),
            ManageState::Status => {
                vec![
                    Action::new(ActionType::Equip, "Equip".text_blue()),
                    Action::new(ActionType::Continue, "Continue".text_green()),
                ]
            }
        }
    }
    fn handle_action(
        &mut self,
        action: Action,
        game_state: &mut GameState,
        player: &mut Player,
    ) -> EventResponse {
        match action.class {
            ActionType::Equip => {
                self.state = ManageState::Equip;
                EventResponse::new(None, None)
            }
            ActionType::EquipItem => {
                player.equip_item_by_name(action.name);

                self.state = ManageState::Status;
                EventResponse::new(None, None)
            }
            ActionType::Continue => {
                let next_event = VisitEvent::build(game_state);
                EventResponse::new(Some(next_event), None)
            }
            _ => panic!("Unhandled action when handling action."),
        }
    }

    fn get_event_loop(&mut self) -> Option<&mut dyn EventLoop> {
        None
    }
}
