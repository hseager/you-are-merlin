#[cfg(test)]
mod tests {
    use you_are_merlin::item::*;
    use you_are_merlin::item::{
        armour::Armour, artifact::Artifact, quest_item::QuestItem, weapon::Weapon,
    };
    use you_are_merlin::text_format::TextFormatter;

    #[test]
    fn test_common_item_display_info() {
        let item = Armour {
            name: "Nightingale Amour".to_string(),
            rarity: ItemRarity::Common,
            max_life: 10,
            block: 0,
            parry: 0.0,
            dodge: 0.0,
        };

        print!("{}", item.display_info());
    }

    #[test]
    fn test_rarity_display_info() {
        let armour = Armour {
            name: "Test Armour".to_string(),
            rarity: ItemRarity::Legendary,
            max_life: 100,
            block: 50,
            parry: 2.4,
            dodge: 3.6,
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
}
