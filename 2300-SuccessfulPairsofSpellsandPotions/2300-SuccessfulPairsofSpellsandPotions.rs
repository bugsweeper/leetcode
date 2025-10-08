// Last updated: 08.10.2025, 09:15:44
impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        let mut potions = potions;
        potions.sort_unstable();
        let success = success as u64;
        spells
            .into_iter()
            .map(|spell| {
                let successful_potion = success.div_ceil(spell as u64);
                let first_successful_potion_index =
                    potions.partition_point(|&potion| (potion as u64) < successful_potion);
                (potions.len() - first_successful_potion_index) as i32
            })
            .collect()
    }
}