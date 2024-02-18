pub enum Location {
    CamelotCastle,
    CamelotInn,
    ForestOfBalor,
    DarklingWoods,
    TheWhiteMountains,
    EaldorVillage,
}

impl Location {
    pub fn as_str(&self) -> &'static str {
        match self {
            Location::CamelotCastle => "Camelot Castle",
            Location::CamelotInn => "Camelot Inn",
            Location::ForestOfBalor => "The Forest Of Balor",
            Location::DarklingWoods => "Darkling Woods",
            Location::TheWhiteMountains => "The White Mountains",
            Location::EaldorVillage => "Ealdor Village",
        }
    }
}
