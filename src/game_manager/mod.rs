use colored::Colorize;

use crate::{
    actions::{
        get_battle_actions, get_locations_as_actions, get_quest_actions, get_visiting_actions,
        Action, ActionType,
    },
    characters::Player,
    config::{PLAYER_ATTACK, PLAYER_LIFE},
    encounter::Encounter,
    player_state::PlayerState,
    theme::load_theme,
    utilities::get_random_array_index,
    world::World,
};

mod battle_manager;
mod world_builder;

pub struct GameState {
    pub player: Player,
    pub state: PlayerState,
    pub world: World,
    pub actions: Vec<Action>,
}

impl GameState {
    pub fn get_current_prompt(&self) -> () {
        self.state.get_prompt(self);

        println!("{}", &self.get_actions_display_list());
    }

    pub fn handle_action(&mut self, search: &str) -> () {
        match &self.find_action(search) {
            Some(action) => match action.class {
                ActionType::Travel => self.state = PlayerState::Travelling,
                ActionType::Explore => match self.world.get_current_encounter() {
                    Encounter::Battle(_) => self.state = PlayerState::Battle,
                    Encounter::BossFight(_) => self.state = PlayerState::Battle,
                    Encounter::Quest(_) => self.state = PlayerState::Quest,
                },
                ActionType::MoveToLocation => {
                    self.state = PlayerState::Visiting;
                    self.world.current_location = self
                        .world
                        .locations
                        .iter()
                        .position(|location| location.name.to_lowercase() == search.to_lowercase())
                        .unwrap();
                }
                ActionType::Attack => battle_manager::handle_battle(self),
                ActionType::Run => {
                    self.state = PlayerState::Visiting;

                    self.world
                        .locations
                        .get_mut(self.world.current_location)
                        .unwrap()
                        .reset_encounters();
                }
                ActionType::Continue => {
                    println!("You acknowledge their request and continue exploring the area.");

                    self.world
                        .locations
                        .get_mut(self.world.current_location)
                        .unwrap()
                        .go_to_next_encounter();

                    self.state = PlayerState::Battle;
                }
            },
            None => println!("This isn't the time to use {}!", search),
        }

        self.actions = self.get_actions(&self.state);
    }

    fn find_action(&self, search: &str) -> Option<&Action> {
        self.actions
            .iter()
            .find(|action| action.name.trim().to_lowercase() == search.to_lowercase())
    }

    // TODO map this in PlayerState rather than here
    fn get_actions(&self, state: &PlayerState) -> Vec<Action> {
        match state {
            PlayerState::Visiting => get_visiting_actions(),
            PlayerState::Battle => get_battle_actions(),
            PlayerState::Quest => get_quest_actions(),
            PlayerState::Travelling => get_locations_as_actions(&self.world.locations),
            _ => vec![],
        }
    }

    fn get_actions_display_list(&self) -> String {
        self.get_actions(&self.state)
            .iter()
            .map(|action| action.display_name())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

pub fn init_game() -> GameState {
    let theme = load_theme();
    let locations = world_builder::build_world(&theme);

    println!("{:#?}", locations);

    let player = Player {
        name: theme.main_character.bold(),
        life: PLAYER_LIFE,
        attack: PLAYER_ATTACK,
    };

    let world: World = World {
        name: &theme.world_name,
        current_location: get_random_array_index(&locations),
        locations,
    };

    GameState {
        player,
        state: PlayerState::Visiting,
        world,
        actions: get_visiting_actions(),
    }
}
