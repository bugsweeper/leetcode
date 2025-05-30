// Last updated: 30.05.2025, 14:26:59
impl Solution {
    pub fn sort_by_bits(arr: Vec<i32>) -> Vec<i32> {
        let mut arr = arr;
        arr.sort_by_cached_key(|value| (value.count_ones(), *value));
        arr
    }
}