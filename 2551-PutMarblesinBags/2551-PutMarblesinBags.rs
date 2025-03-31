// Last updated: 31.03.2025, 11:32:32
impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let mut borders = weights.iter().zip(weights.iter().skip(1)).map(|(&left, &right)| (left + right) as i64).collect::<Vec<_>>();
        borders.sort_unstable();
        let k = k as usize - 1;
        let different_part_size = k.min(borders.len() - k);
        borders.iter().rev().take(different_part_size).sum::<i64>() - borders.iter().take(different_part_size).sum::<i64>()
    }
}