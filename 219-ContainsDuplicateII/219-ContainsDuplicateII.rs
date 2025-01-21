impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut seen = std::collections::HashMap::with_capacity(nums.len());
        for (index, item) in nums.into_iter().enumerate() {
            if let Some(prev_index) = seen.insert(item, index) {
                if index - prev_index <= k {
                    return true;
                }
            }
        }
        false
    }
}