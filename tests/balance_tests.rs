mod tests {

    use colored::Colorize;
    use you_are_merlin::{
        characters::{enemy::Enemy, fighter::Fighter, player::Player},
        config::{ENEMY_BOSS_STATS, ENEMY_EASY_STATS, ENEMY_HARD_STATS, ENEMY_MEDIUM_STATS},
    };

    fn display_stats(name: &String, life: &i16, power: &u16, attack_speed_milliseconds: &u16) {
        let dps = power / (attack_speed_milliseconds / 1000);
        print!(
            "\n\
            {}\n\
            DPS: {}\n\
            Life: {}\n\
            Life/DPS: {}\n\
            ",
            name.bold(),
            dps,
            life,
            dps as i16 * life
        );
    }

    #[test]
    fn test_player_base_dps() {
        let player = Player::new(String::from("Player"));

        display_stats(
            &player.name,
            player.life(),
            &player.stats.power,
            &player.attack_speed_as_milliseconds(),
        );
    }

    #[test]
    fn test_enemy_dps() {
        let enemy_types = [
            (ENEMY_EASY_STATS, "Easy"),
            (ENEMY_MEDIUM_STATS, "Medium"),
            (ENEMY_HARD_STATS, "Hard"),
            (ENEMY_BOSS_STATS, "Boss"),
        ];

        for enemy_type in enemy_types {
            let enemy = Enemy::new(
                format!("Enemy {}", enemy_type.1),
                "",
                enemy_type.0 .0,
                enemy_type.0 .1,
                enemy_type.0 .2,
            );

            display_stats(
                &enemy.name,
                enemy.life(),
                &enemy.stats.power,
                &enemy.attack_speed_as_milliseconds(),
            );
        }
    }
}