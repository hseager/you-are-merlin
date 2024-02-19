use colored::{Color, Colorize};

pub enum Location {
    CamelotCastle,
    CamelotInn,
    ForestOfBalor,
    DarklingWoods,
    TheWhiteMountains,
    EaldorVillage,
}

pub struct LocationItem {
    pub class: Location,
    pub label: &'static str,
    pub label_color: Color
}

impl LocationItem {
    pub fn display_label(&self) -> String {
        self.label.color(self.label_color).to_string()
    }
}

pub fn get_all_locations() -> Vec<LocationItem> {
    vec![
        LocationItem {
            class: Location::CamelotCastle,
            label: "Camelot Castle",
            label_color: Color::BrightBlue
        },
        LocationItem {
            class: Location::CamelotInn,
            label: "Camelot Inn",
            label_color: Color::BrightCyan
        },
        LocationItem {
            class: Location::ForestOfBalor,
            label: "The Forest Of Balor",
            label_color: Color::BrightGreen
        },
        LocationItem {
            class: Location::DarklingWoods,
            label: "Darkling Woods",
            label_color: Color::BrightMagenta
        },
        LocationItem {
            class: Location::TheWhiteMountains,
            label: "The White Mountains",
            label_color: Color::BrightRed
        },
        LocationItem {
            class: Location::EaldorVillage,
            label: "Ealdor Village",
            label_color: Color::BrightYellow
        }
    ]
}

pub fn get_locations_list() -> String {
    let locations = get_all_locations();
    
    locations
        .iter()
        .map(|location| location.display_label())
        .collect::<Vec<String>>()
        .join(", ")
}