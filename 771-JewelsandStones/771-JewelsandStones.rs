// Last updated: 28.04.2025, 14:07:02
impl Solution {
    pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
        let mut is_jewel = vec![false; u8::MAX as usize];
        for &jewel in jewels.as_bytes() {
            is_jewel[jewel as usize] = true;
        }
        stones
            .as_bytes()
            .iter()
            .copied()
            .filter(|&stone| is_jewel[stone as usize])
            .count() as i32
    }
}