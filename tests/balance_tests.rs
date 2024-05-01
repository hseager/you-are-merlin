mod tests {

    use you_are_merlin::characters::{fighter::Fighter, player::Player};

    fn display_dps(name: &String, power: &u16, attack_speed_milliseconds: &u16) {
        print!(
            "{} DPS: {}",
            name,
            (power / (attack_speed_milliseconds / 1000)) as i32
        );
    }

    #[test]
    fn test_player_base_dps() {
        let player = Player::new(String::from("Player"));

        display_dps(
            &player.name,
            &player.stats.power,
            &player.attack_speed_as_milliseconds(),
        );
    }
}
