use colored::{Color, Colorize};

#[derive(Clone)]
pub enum Location {
    CamelotCastle,
    CamelotInn,
    ForestOfBalor,
    DarklingWoods,
    TheWhiteMountains,
    EaldorVillage,
}

#[derive(Clone)]
pub struct LocationItem {
    pub class: Location,
    pub label: &'static str,
    pub label_color: Color
}

impl LocationItem {
    pub fn new(class: Location, label: &'static str, label_color: Color) -> Self {
        Self {
            class,
            label,
            label_color
        }
    }

    pub fn display_label(&self) -> String {
        self.label.color(self.label_color).to_string()
    }
}

const ALL_LOCATIONS: [LocationItem; 6] = [
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
        label_color: Color::White
    },
    LocationItem {
        class: Location::EaldorVillage,
        label: "Ealdor Village",
        label_color: Color::BrightYellow
    }
];

pub fn get_location(search: &str) -> Option<&LocationItem> {
    ALL_LOCATIONS.iter().find(|location| location.label.to_lowercase() == search.to_lowercase())
}

pub fn get_locations_display_list() -> String {
    ALL_LOCATIONS
        .iter()
        .map(|location| location.display_label())
        .collect::<Vec<String>>()
        .join(", ")
}