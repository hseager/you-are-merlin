use std::io;

use theme::select_theme;
use wasm_bindgen::prelude::*;

use game_data::GameData;
use game_state::GameState;
use player_state::PlayerState;
use web_sys::CustomEvent;

use crate::theme::get_theme;

mod actions;
mod battle_manager;
mod characters;
mod config;
mod game_data;
mod game_state;
mod items;
mod player_state;
mod prompts;
mod theme;
mod utilities;

// #[wasm_bindgen]
// pub fn initialize_game(theme: String) {
//     // Call the JavaScript functions to select theme and get theme data
//     let theme_data = js_get_theme(theme);
//     let game_data = GameData::new(theme_data);
//     let game_state = GameState::new(game_data);

//     // Use game state
//     js_print_player_name(&game_state.player.name);
// }

// fn js_get_game_data(theme: String) -> JsValue {
//     let theme_data = js_get_theme(theme);
//     let game_data = GameData::new(theme_data);

//     wasm_bindgen::JsValue::from(game_data)
// }

// Define your GameData and GameState structs here

// fn js_get_theme(selection: String) -> Theme {
//     // Call JavaScript to get theme data based on selection
//     // Example:
//     // wasm_bindgen::JsValue::from(js_get_theme(selection))
//     unimplemented!()
// }

// fn js_print_player_name(name: &str) {
//     // Call JavaScript to print player name
//     // Example:
//     // js_print_player_name(name)
//     unimplemented!()
// }

// #[wasm_bindgen]
// pub struct GameDataContainer {
//     game_data: GameData,
// }

// #[wasm_bindgen]
// impl GameDataContainer {
//     #[wasm_bindgen(constructor)]
//     pub fn new(theme: &str) -> GameDataContainer {
//         let theme_data = get_theme(theme.to_string());

//         GameDataContainer {
//             game_data: GameData::new(theme_data),
//         }
//     }

//     pub fn get_world_name(&self) -> String {
//         self.game_data.world_name.to_string()
//     }

//     pub fn get_character_name(&self) -> String {
//         self.game_data.main_character.to_string()
//     }

//     pub fn get_data(&self) -> String {
//         self.game_data.to_string()
//     }
// }

// #[wasm_bindgen]
// pub struct GameStateContainer {
//     game_state: GameState<'static>,
// }

// #[wasm_bindgen]
// impl GameStateContainer {
//     pub fn new(game_data_container: GameDataContainer) -> GameStateContainer {
//         let game_data = game_data_container.game_data.clone();
//         let game_state = GameState::new(game_data);

//         GameStateContainer { game_state }
//     }
// }

#[wasm_bindgen(module = "/js/wasm-bindings.js")]
extern "C" {
    // fn trigger_prompt_event(prompt: String) -> String;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

#[wasm_bindgen]
pub fn start(theme: String) -> Result<(), JsValue> {
    let theme_data = get_theme(theme);
    let game_data = GameData::new(theme_data);
    let mut game_state = GameState::new(&game_data);

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let terminal = document
        .query_selector("#terminal")
        .expect("Unable to find terminal element.")
        .unwrap();

    terminal.set_text_content(Some(""));

    terminal
        .insert_adjacent_text(
            "afterbegin",
            format!("You are {}.\n", &game_state.player.name).as_str(),
        )
        .expect("Unable to add update prompt");

    terminal
        .insert_adjacent_text(
            "beforeend",
            format!("{}\n", game_state.get_prompt()).as_str(),
        )
        .expect("Unable to add update prompt");

    terminal
        .insert_adjacent_text(
            "beforeend",
            format!("{}\n", game_state.get_actions_display_list()).as_str(),
        )
        .expect("Unable to add update prompt");

    // log(format!("You are {}.", &game_state.player.name));
    // trigger_prompt_event(game_state.get_prompt());

    // let prompt_update_event = CustomEvent::new("prompt-update");

    Ok(())

    // loop {
    //     let mut input = String::new();

    //     if let PlayerState::GameOver = game_state.state {
    //         println!("Game Over...");
    //         break;
    //     }

    //     if let PlayerState::Win = game_state.state {
    //         println!("You Win!");
    //         break;
    //     }

    //     game_state.get_prompt();

    //     io::stdin()
    //         .read_line(&mut input)
    //         .expect("Failed to read line");

    //     game_state.handle_action(input.trim());
    // }
}
