use crate::{
    actions::{get_exploring_actions, get_visiting_actions, Action, ActionType},
    encounter::{Encounter, EncounterType, Enemy},
    location::Location,
    player::Player,
    theme::{load_theme, Theme, ThemeLocation},
    utilities::{get_random_array_index, map_text_color},
};

pub struct GameState {
    pub player: Player,
    pub state: State,
    pub current_location: usize,
    pub locations: Vec<Location>,
    pub actions: Vec<Action>,
}

pub enum State {
    Travelling,
    Visiting,
    Exploring,
}

impl GameState {
    fn get_current_location(&self) -> &Location {
        &self.locations[self.current_location] // TODO check out of bounds
    }

    pub fn get_current_prompt(&self) -> () {
        let current_location = &self.get_current_location();
        let current_encounter = &self.get_current_encounter();

        match &self.state {
            State::Visiting => {
                println!(
                    "You are currently visiting {}. {}",
                    current_location.display_name(),
                    current_location.description
                );
                println!("What would you like to do?")
            }
            State::Travelling => println!("Where would you like to go?"),
            State::Exploring => match current_encounter.class {
                EncounterType::Battle => {
                    let Enemy { name, life, attack } = current_encounter.enemy;

                    println!(
                        "A wild {} appears! (life: {}, attack: {})",
                        name, life, attack
                    )

                    // let enemy = self
                    //     .locations
                    //     .get(0)
                    //     .unwrap()
                    //     .encounters
                    //     .get(0)
                    //     .unwrap()
                    //     .enemy;

                    // println!(
                    //     "A wild {} appears! (life: {}, attack: {})",
                    //     enemy.name, enemy.life, enemy.attack
                    // )
                }
            },
        }

        println!("{}", &self.get_actions_display_list());
    }

    pub fn handle_action(&mut self, search: &str) -> () {
        match &self.find_action(search) {
            Some(action) => match action.class {
                ActionType::Travel => {
                    self.state = State::Travelling;
                    self.actions = self.get_actions(&self.state);
                }
                ActionType::Explore => {
                    self.state = State::Exploring;
                    self.actions = self.get_actions(&self.state);
                }
                ActionType::MoveToLocation => {
                    self.state = State::Visiting;
                    self.actions = self.get_actions(&self.state);
                    self.current_location = self
                        .locations
                        .iter()
                        .position(|location| location.name.to_lowercase() == search.to_lowercase())
                        .unwrap();
                }
                ActionType::Attack => {
                    let current_encounter = &mut self.get_current_encounter_mut();

                    current_encounter.enemy.life -= self.player.attack;

                    println!("You attack for {} damage.", self.player.attack)
                }
                ActionType::Run => {
                    self.state = State::Visiting;
                    self.actions = self.get_actions(&self.state);
                }
            },
            None => println!("This isn't the time to use {}!", search),
        }
    }

    fn find_action(&self, search: &str) -> Option<&Action> {
        self.actions
            .iter()
            .find(|action| action.name.trim().to_lowercase() == search.to_lowercase())
    }

    fn get_actions(&self, state: &State) -> Vec<Action> {
        let locations: Vec<Action> = self
            .locations
            .iter()
            .map(|location| {
                Action::new(
                    ActionType::MoveToLocation,
                    &location.name,
                    location.name_color,
                )
            })
            .collect();

        match state {
            State::Visiting => get_visiting_actions(),
            State::Exploring => get_exploring_actions(),
            State::Travelling => locations,
        }
    }

    fn get_actions_display_list(&self) -> String {
        self.get_actions(&self.state)
            .iter()
            .map(|action| action.display_name())
            .collect::<Vec<String>>()
            .join(", ")
    }

    fn get_current_encounter(&self) -> &Encounter {
        let current_location = &self.get_current_location();
        &current_location.encounters[current_location.current_encounter]
    }

    fn get_current_encounter_mut(&mut self) -> &mut Encounter {
        let current_location = &mut self.locations[self.current_location];
        &mut current_location.encounters[current_location.current_encounter]
    }
}

pub fn init_game() -> GameState {
    let theme = load_theme();
    let name = theme.main_character;
    let locations = build_game_locations(&theme);

    let player = Player {
        name,
        life: 100,
        attack: 4,
    };

    GameState {
        player,
        state: State::Visiting,
        current_location: get_random_array_index(&locations),
        locations,
        actions: get_visiting_actions(),
    }
}

fn build_game_locations(theme: &Theme) -> Vec<Location> {
    let mut locations = Vec::new();
    for (i, theme_location) in theme.locations.iter().enumerate() {
        locations.push(Location {
            name: theme_location.name,
            description: theme_location.description,
            name_color: map_text_color(i),
            current_encounter: 0,
            encounters: build_game_encounters(theme_location),
        })
    }
    locations
}

fn build_game_encounters(theme_location: &ThemeLocation) -> Vec<Encounter> {
    theme_location
        .enemies
        .map(|enemy| Encounter {
            class: EncounterType::Battle,
            enemy: Enemy {
                name: enemy.name,
                attack: enemy.attack,
                life: enemy.life,
            },
            reward: 2,
        })
        .to_vec()
}
