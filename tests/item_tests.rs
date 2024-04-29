#[cfg(test)]
mod tests {
    use you_are_merlin::item::armour::Armour;
    use you_are_merlin::item::artifact::Artifact;
    use you_are_merlin::item::quest_item::Weapon;
    use you_are_merlin::item::weapon::Weapon;
    use you_are_merlin::item::{Item, ItemRarity};
    use you_are_merlin::text_format::TextFormatter;

    #[test]
    fn test_rarity_display_info() {
        let armour = Armour {
            name: "Test Armour".to_string(),
            rarity: ItemRarity::Rare,
            max_life: 100,
            block: 50,
            parry_chance: 0,
            dodge_chance: 0,
        };

        print!("{}", armour.display_info());
    }

    #[test]
    fn test_armour_generation_display_info() {
        let item = Armour::new("Judgement Armor".to_string());

        print!("{}", item.display_info());
    }

    #[test]
    fn test_weapon_generation_display_info() {
        let item = Weapon::new("Thunderfury, Blessed Blade of the Windseeker".to_string());

        print!("{}", item.display_info());
    }

    #[test]
    fn test_artifact_generation_display_info() {
        let item = Artifact::new("Onyxia Scale Cloak".to_string());

        print!("{}", item.display_info());
    }

    #[test]
    fn test_quest_item_display_info() {
        let item = QuestItem::new("Orb of Mayhem".to_string());

        print!("{}", item.display_info());
    }

    #[test]
    fn test_display_info() {
        let armour = Armour {
            name: "Test Armour".to_string(),
            rarity: ItemRarity::Rare,
            max_life: 100,
            block: 50,
            parry_chance: 0,
            dodge_chance: 0,
        };

        let expected_output = format!(
            "\n{} - (Rare)\n\
            - Max Life: 100\n\
            - Block: 50",
            "Test Armour".text_bold()
        );

        assert_eq!(armour.display_info(), expected_output);
    }
}
