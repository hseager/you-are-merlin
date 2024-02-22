// use colored::Color;

// use crate::{
//     actions::{Action, ActionItem},
//     location,
// };

// pub enum Event {
//     Visiting,
//     Travelling,
// }

// pub struct EventItem {
//     pub class: Event,
//     pub location: LocationItem,
//     pub actions: Vec<ActionItem>,
//     // next: EventItem,
// }

// impl EventItem {
//     pub fn get_actions_display_list(&self) -> String {
//         self.actions
//             .iter()
//             .map(|action| action.display_label())
//             .collect::<Vec<String>>()
//             .join(", ")
//     }

//     pub fn has_action(&self, search: &str) -> bool {
//         self.actions
//             .iter()
//             .any(|action| action.label.to_lowercase() == search.to_lowercase())
//     }

//     // TODO better error handling
//     fn find_action(&mut self, search: &str) -> &ActionItem {
//         match self
//             .actions
//             .iter()
//             .find(|action| action.label.to_lowercase() == search.to_lowercase())
//         {
//             Some(action) => action,
//             None => panic!("Couldn't find matched action"),
//         }
//     }

//     pub fn execute_action(&mut self, search: &str) -> () {
//         let action = self.find_action(search);

//         match action.class {
//             Action::Explore => {
//                 self.actions = vec![
//                     ActionItem::new(Action::Attack, "Attack", Color::Red),
//                     ActionItem::new(Action::CastSpell, "Cast Spell", Color::Magenta),
//                 ];
//                 println!(
//                     "You begin to explore {}, but a giant spider appears.",
//                     self.location.display_label()
//                 );
//                 println!("Options: {}", self.get_actions_display_list());
//             }
//             Action::Travel => {
//                 self.class = Event::Travelling;
//                 println!(
//                     "Where would you like to go? {}",
//                     get_locations_display_list()
//                 )
//             }
//             Action::Attack => {
//                 println!("You attack with your dagger. You did {} damage.", 3)
//             }
//             Action::CastSpell => {
//                 println!("Which spell do you cast?")
//             }
//         }
//     }
// }
