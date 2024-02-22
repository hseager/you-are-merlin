pub struct Theme {
    main_character: String,
    characters: [String; 5],
    locations: [Location; 4],
    items: [String; 10],
    enemies: [Enemy; 6]
}

struct Location {
    name: String,
    description: String
}

struct Enemy {
    name: String,
    life: i16,
    attack: i8
}
