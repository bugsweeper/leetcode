impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = std::collections::HashMap::with_capacity(nums.len());
        for (index, &num) in nums.iter().enumerate() {
            let other = target - num;
            if seen.contains_key(&other) {
                return vec![index as i32, seen[&other] as i32];
            }
            seen.insert(num, index);
        }
        Vec::new()
    }
}