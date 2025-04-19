// Last updated: 19.04.2025, 13:08:48
impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut count = 0;
        let first = *nums.first().unwrap();
        let last = *nums.last().unwrap();
        if first * 2 > upper || last * 2 < lower {
            return 0;
        }
        for (i, &num_i) in nums.iter().enumerate() {
            let nums_i = &nums[i + 1..];
            let (from, to) = (lower - num_i, upper - num_i);
            let from_j = nums_i.partition_point(|&num| num < from);
            let to_j = nums_i[from_j..].partition_point(|&num| num <= to) + from_j;
            count += (to_j - from_j) as i64;
        }
        count
    }
}