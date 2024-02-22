use crate::{
    // actions::{Action, ActionItem},
    // event::{Event, EventItem},
    location::Location,
    player::Player,
    theme::{load_theme, Theme},
    utilities::{get_random_array_index, get_random_color},
};

pub struct GameState {
    pub player: Player,
    pub state: State,
    pub current_location: usize,
    pub locations: [Location; 6],
}

pub enum State {
    Visiting,
    Exploring,
}

impl GameState {
    fn get_current_location(&self) -> &Location {
        &self.locations[self.current_location] // TODO check out of bounds
    }

    pub fn get_current_prompt(&self) -> () {
        let current_location = self.get_current_location();

        match &self.state {
            State::Visiting => {
                println!(
                    "You are currently visiting {}. {}",
                    current_location.display_name(),
                    current_location.description
                );
                println!("What would you like to do?")
            }
            State::Exploring => {
                println!("You are exploring");
            }
        }
    }

    pub fn handle_action(&self, action: &str) -> () {
        println!("You chose: {}", action);
    }
}

pub fn init_game() -> GameState {
    let theme = load_theme();

    // Create Player
    let player = Player {
        name: theme.main_character,
        life: 100,
        attack: 4,
    };

    // let initial_actions = vec![
    //     ActionItem::new(Action::Explore, "Explore", Color::Yellow),
    //     ActionItem::new(Action::Travel, "Travel", Color::Blue),
    // ];

    // // TODO fix unwrap & clone
    // let inital_event = EventItem {
    //     class: Event::Visiting,
    //     location: get_location("The White Mountains").unwrap().clone(),
    //     actions: initial_actions,
    // };

    GameState {
        player,
        state: State::Visiting,
        current_location: get_random_array_index(&theme.locations),
        locations: build_game_world(theme),
    }
}

fn build_game_world(theme: Theme) -> [Location; 6] {
    theme.locations.map(|theme_location| Location {
        name: theme_location.name,
        description: theme_location.description,
        name_color: get_random_color(),
    })
}
