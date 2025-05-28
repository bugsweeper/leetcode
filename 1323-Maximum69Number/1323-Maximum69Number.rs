// Last updated: 28.05.2025, 16:46:04
impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut sorted = arr.clone();
        sorted.sort_unstable();
        sorted.dedup();
        arr.into_iter()
            .map(|value| sorted.partition_point(|&sorted| sorted < value) as i32 + 1)
            .collect()
    }
}