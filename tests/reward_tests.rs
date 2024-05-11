#[cfg(test)]
mod tests {
    use you_are_merlin::item::reward_type::RewardType;
    use you_are_merlin::item::ItemRarity;

    #[test]
    fn test_roll_rarity_distribution() {
        const NUM_ITERATIONS: usize = 100000;

        let mut common_count = 0;
        let mut rare_count = 0;
        let mut epic_count = 0;
        let mut legendary_count = 0;

        for _ in 0..NUM_ITERATIONS {
            let reward_type = RewardType::QuestReward;

            let rarity = reward_type.roll_rarity();

            match rarity {
                ItemRarity::Common => common_count += 1,
                ItemRarity::Rare => rare_count += 1,
                ItemRarity::Epic => epic_count += 1,
                ItemRarity::Legendary => legendary_count += 1,
            }
        }

        let total_iterations = NUM_ITERATIONS as f64;
        let common_percentage = (common_count as f64 / total_iterations) * 100.0;
        let rare_percentage = (rare_count as f64 / total_iterations) * 100.0;
        let epic_percentage = (epic_count as f64 / total_iterations) * 100.0;
        let legendary_percentage = (legendary_count as f64 / total_iterations) * 100.0;

        println!("Common: {:.2}%", common_percentage);
        println!("Rare: {:.2}%", rare_percentage);
        println!("Epic: {:.2}%", epic_percentage);
        println!("Legendary: {:.2}%", legendary_percentage);

        // Assert: Compare the percentages against the expected reward chances
        // Add assertions here based on the expected distribution of rarities
        // For example:
        // assert!((common_percentage - expected_common_percentage).abs() < tolerance);
        // assert!((rare_percentage - expected_rare_percentage).abs() < tolerance);
        // assert!((epic_percentage - expected_epic_percentage).abs() < tolerance);
        // assert!((legendary_percentage - expected_legendary_percentage).abs() < tolerance);
    }
}
