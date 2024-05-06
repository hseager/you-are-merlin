#[cfg(test)]
mod tests {
    use you_are_merlin::characters::enemy::Enemy;
    use you_are_merlin::characters::fighter::Fighter;
    use you_are_merlin::characters::player::Player;
    use you_are_merlin::characters::stats::Stats;
    use you_are_merlin::config::*;
    use you_are_merlin::item::*;
    use you_are_merlin::item::{armour::Armour, weapon::Weapon};

    const PLAYER_LIFE_VALUE: i16 = 100;

    const ENEMY_ATTACK: u16 = 10;

    const BLOCK_VALUE: u16 = 5;

    fn setup_player() -> Player {
        let stats = Stats {
            max_life: PLAYER_LIFE_VALUE,
            life: PLAYER_LIFE_VALUE,
            power: PLAYER_ATTACK,
            attack_speed: PLAYER_ATTACK_SPEED,
            block: PLAYER_BLOCK,
            crit_multiplier: PLAYER_CRIT_MULTI,
            crit_chance: PLAYER_CRIT_CHANCE,
            parry: PLAYER_PARRY_CHANCE,
            dodge: PLAYER_DODGE_CHANCE,
        };

        let weapon = Weapon {
            name: String::from("Weapon"),
            rarity: ItemRarity::Legendary,
            power: 1,
            attack_speed: 300,
            crit_chance: 0.0,
            crit_multiplier: 0.0,
        };
        let armour = Armour {
            name: String::from("Armour"),
            rarity: ItemRarity::Legendary,
            max_life: 0,
            block: BLOCK_VALUE,
            parry: 5.0,
            dodge: 5.0,
        };

        let equipment = Equipment {
            armour: Some(Box::new(armour)),
            weapon: Some(Box::new(weapon)),
            artifact: None,
        };

        Player {
            name: String::from("Player"),
            stats,
            inventory: Vec::new(),
            equipment,
        }
    }

    fn setup_enemy() -> Enemy {
        let stats = Stats {
            max_life: 100,
            life: 100,
            power: ENEMY_ATTACK,
            attack_speed: 50,
            block: 0,
            crit_multiplier: 0.0,
            crit_chance: 0.0,
            parry: 0.0,
            dodge: 0.0,
        };

        Enemy {
            name: String::from("Enemy"),
            description: "Description",
            stats,
        }
    }

    #[test]
    fn test_block_prevents_damage() {
        let mut player = setup_player();
        let enemy = setup_enemy();

        enemy.attack(&mut player);

        let blocked_attack_value = (PLAYER_LIFE - (ENEMY_ATTACK as i16 - BLOCK_VALUE as i16) - 2)
            as i16
            ..=(PLAYER_LIFE - (ENEMY_ATTACK as i16 - BLOCK_VALUE as i16) + 2) as i16;

        println!("{:?}", blocked_attack_value);
        println!("{}", player.life());

        assert!(
            blocked_attack_value.contains(&(player.life())),
            "Expect that player block prevents damage."
        );
    }

    #[test]
    fn test_block_more_than_damage_prevents_all_damage() {
        let mut player = setup_player();
        let enemy = setup_enemy();

        player.stats.block = 100;

        enemy.attack(&mut player);

        let blocked_attack_value = (PLAYER_LIFE - (ENEMY_ATTACK as i16 - BLOCK_VALUE as i16) - 2)
            as i16
            ..=(PLAYER_LIFE - (ENEMY_ATTACK as i16 - BLOCK_VALUE as i16) + 2) as i16;

        println!("{:?}", blocked_attack_value);
        println!("{}", player.life());

        assert_eq!(player.life(), PLAYER_LIFE_VALUE);
    }
}
