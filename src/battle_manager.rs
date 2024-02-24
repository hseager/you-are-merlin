use crate::location::Location;

pub fn handle_battle(location: &mut Location, player_attack: i16) {
    let encounter = &mut location.encounters[location.current_encounter]; // TODO Check out of bounds

    location.current_encounter += 1;

    encounter.enemy.life -= player_attack;

    println!("You attack for {} damage.", player_attack);
}
