#[cfg(test)]
mod tests {
    use you_are_merlin::item::armour::Armour;
    use you_are_merlin::item::{Item, ItemRarity};
    use you_are_merlin::text_format::TextFormatter;

    #[test]
    fn test_run_display_info() {
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
