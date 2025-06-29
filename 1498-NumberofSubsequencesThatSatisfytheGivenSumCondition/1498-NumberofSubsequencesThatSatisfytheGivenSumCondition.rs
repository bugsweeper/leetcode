// Last updated: 29.06.2025, 05:29:13
const MODULO: i32 = 1_000_000_007;

impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut power_of_two = Vec::with_capacity(nums.len() + 1);
        let mut product = 1;
        power_of_two.push(product);
        for _ in 0..nums.len() {
            product = (product << 1) % MODULO;
            power_of_two.push(product);
        }
        let mut num_subsequences = 0;
        for (right_index, &right_value) in nums.iter().enumerate() {
            if right_value >= target {
                break;
            }
            let right_count = right_index + 1;
            let min_overflow_value = target - right_value + 1;
            let left_acceptable_count =
                nums[..right_count].partition_point(|&left| left < min_overflow_value);
            if left_acceptable_count == right_count {
                num_subsequences =
                    (num_subsequences + power_of_two[right_index]).rem_euclid(MODULO);
            } else {
                num_subsequences = (num_subsequences + power_of_two[right_index]
                    - power_of_two[right_index - left_acceptable_count])
                    .rem_euclid(MODULO);
            }
        }
        num_subsequences
    }
}