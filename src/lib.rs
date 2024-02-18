enum Location {
  CamelotCastle,
  CamelotInn,
  ForestOfBalor,
  DarklingWoods,
  TheWhiteMountains,
  EaldorVillage
}

pub struct GameState {
  pub current_location: Location,
}