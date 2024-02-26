use crate::game_manager::GameState;

pub fn handle_battle(game_state: &mut GameState) {
    let location = game_state.locations.get_mut(game_state.current_location);

    if let Some(location) = location {
        if let Some(encounter) = location.encounters.get_mut(location.current_encounter) {
            let player = &mut game_state.player;

            if encounter.enemy.life - player.attack <= 0 {
                println!("You defeated {}!", encounter.enemy.name);
                location.current_encounter += 1;
                encounter.enemy.life = 0;
            } else {
                encounter.enemy.life -= player.attack;
                println!(
                    "You attack {} for {} damage.",
                    encounter.enemy.name, player.attack
                );

                player.life -= encounter.enemy.attack;
                println!(
                    "{} attacks you for {} damage. (Your life: {})",
                    encounter.enemy.name, encounter.enemy.attack, player.life
                );
            }
        } else {
            println!("No more encounters.");
        }
    } else {
        println!("Couldn't find location.");
    }
}
