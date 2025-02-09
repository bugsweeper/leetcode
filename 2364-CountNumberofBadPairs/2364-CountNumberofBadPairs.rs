impl Solution {
    pub fn count_bad_pairs(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut base_count = std::collections::HashMap::with_capacity(nums.len());
        for (index, num) in nums.into_iter().enumerate() {
            *base_count.entry(num as usize - index).or_insert(0i64) += 1;
        }
        let counts = base_count.into_values().collect::<Vec<_>>();
        let mut result = 0;
        let mut sum: i64 = n as i64;
        for count in counts {
            sum -= count;
            result += count * sum;
        }
        result
    }
}