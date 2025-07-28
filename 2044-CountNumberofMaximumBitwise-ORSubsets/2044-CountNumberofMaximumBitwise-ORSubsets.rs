// Last updated: 28.07.2025, 11:47:43
impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }
        let max = nums.iter().copied().reduce(|acc, num| acc | num).unwrap();
        let mut nums = nums;
        // Processing numbers with more ones in binary representation at the beginning
        // can increase the probability of subset grouping
        // because if max_or_sum is met on only portion of nums,
        // then count of combinations could be increased by number of combinations of remaining nums
        nums.sort_by_cached_key(|num| num.count_ones());
        let mut bits_mask = 1;
        let bits_mask_end = 1 << nums.len();
        let mut count = 0;
        'subsets: while bits_mask < bits_mask_end {
            let mut or_sum = 0;
            for (index, num) in nums.iter().copied().enumerate().rev() {
                let bit_mask = 1 << index;
                if bits_mask & bit_mask != 0 {
                    or_sum |= num;
                    if or_sum == max {
                        count += bit_mask;
                        bits_mask += bit_mask;
                        continue 'subsets;
                    }
                }
            }
            // Didn't match subset, go to next subset anyway
            bits_mask += 1;
        }
        count
    }
}