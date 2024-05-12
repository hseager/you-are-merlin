#[cfg(test)]
mod tests {
    use you_are_merlin::characters::fighter::Fighter;
    use you_are_merlin::characters::player::Player;
    use you_are_merlin::characters::stats::Stats;
    use you_are_merlin::config::*;
    use you_are_merlin::item::*;
    use you_are_merlin::item::{armour::Armour, artifact::Artifact, weapon::Weapon};

    const POWER_VALUE: u16 = 20;
    const ATTACK_SPEED_VALUE: u16 = 20;
    const MAX_LIFE_VALUE: i16 = 20;
    const BLOCK_VALUE: u16 = 2;
    const CRIT_MULTI_VALUE: f32 = 2.0;
    const CRIT_CHANCE_VALUE: f32 = 3.0;
    const PARRY_VALUE: f32 = 4.0;
    const DODGE_VALUE: f32 = 5.0;

    fn setup_player() -> Player {
        let stats = Stats {
            max_life: PLAYER_LIFE,
            life: PLAYER_LIFE,
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
            rarity: ItemRarity::Rare,
            power: POWER_VALUE,
            attack_speed: ATTACK_SPEED_VALUE,
            crit_multiplier: CRIT_MULTI_VALUE,
            crit_chance: CRIT_CHANCE_VALUE,
        };
        let armour = Armour {
            name: String::from("Armour"),
            rarity: ItemRarity::Rare,
            max_life: MAX_LIFE_VALUE,
            block: BLOCK_VALUE,
            parry: PARRY_VALUE,
            dodge: DODGE_VALUE,
        };
        let artifact = Artifact {
            name: String::from("Artifact"),
            rarity: ItemRarity::Rare,
            power: POWER_VALUE,
            attack_speed: ATTACK_SPEED_VALUE,
            max_life: MAX_LIFE_VALUE,
            dodge: DODGE_VALUE,
        };

        let equipment = Equipment {
            armour: Some(Box::new(armour)),
            weapon: Some(Box::new(weapon)),
            artifact: Some(Box::new(artifact)),
        };

        let player = Player {
            name: String::from("Player"),
            stats,
            inventory: Vec::new(),
            equipment,
        };

        player
    }

    #[test]
    fn test_player_power() {
        let player = setup_player();

        assert_eq!(player.power(), PLAYER_ATTACK + (POWER_VALUE * 2))
    }

    #[test]
    fn test_player_attack_speed() {
        let player = setup_player();

        assert_eq!(
            player.attack_speed(),
            PLAYER_ATTACK_SPEED + (ATTACK_SPEED_VALUE * 2)
        )
    }

    #[test]
    fn test_player_crit_multi() {
        let player = setup_player();

        assert_eq!(
            player.crit_multiplier(),
            PLAYER_CRIT_MULTI + CRIT_MULTI_VALUE
        )
    }

    #[test]
    fn test_player_crit_chance() {
        let player = setup_player();

        assert_eq!(player.crit_chance(), PLAYER_CRIT_CHANCE + CRIT_CHANCE_VALUE)
    }

    #[test]
    fn test_player_max_life() {
        let player = setup_player();

        assert_eq!(player.max_life(), PLAYER_LIFE + (MAX_LIFE_VALUE * 2))
    }

    #[test]
    fn test_player_block() {
        let player = setup_player();

        assert_eq!(player.block(), PLAYER_BLOCK + BLOCK_VALUE)
    }

    #[test]
    fn test_player_parry() {
        let player = setup_player();

        assert_eq!(player.parry(), PLAYER_PARRY_CHANCE + PARRY_VALUE)
    }

    #[test]
    fn test_player_dodge() {
        let player = setup_player();

        assert_eq!(player.dodge(), PLAYER_DODGE_CHANCE + (DODGE_VALUE * 2.0))
    }
}
