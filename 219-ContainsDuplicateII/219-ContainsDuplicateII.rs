impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let k = k as usize;
        let mut seen = std::collections::HashSet::with_capacity(k);
        let mut iter = nums.iter();
        for &item in (&mut iter).take(k) {
            if !seen.insert(item) {
                return true;
            }
        }
        for (old_item, &new_item) in nums.iter().zip(iter) {
            if !seen.insert(new_item) {
                return true;
            }
            seen.remove(old_item);
        }
        false
    }
}