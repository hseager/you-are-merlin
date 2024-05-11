mod tests {
    use you_are_merlin::{item::ItemType, theme::theme_data};

    #[test]
    fn test_theme_item_types() {
        let themes = theme_data::get_themes();
        let max_item_type = 9;

        for theme in themes {
            let mut weapon_count = 0;
            let mut armour_count = 0;
            let mut artifact_count = 0;

            for item in theme.1.items {
                match item.item_type {
                    ItemType::Weapon => {
                        weapon_count += 1;
                    }
                    ItemType::Armour => {
                        armour_count += 1;
                    }
                    ItemType::Artifact => {
                        artifact_count += 1;
                    }
                    _ => (),
                }
            }

            println!(
                "{} - Weapons: {} / Armour: {} / Artifacts: {}",
                theme.0, weapon_count, armour_count, artifact_count
            );

            assert!(weapon_count < max_item_type);
            assert!(armour_count < max_item_type);
            assert!(artifact_count < max_item_type);
        }
    }
}
