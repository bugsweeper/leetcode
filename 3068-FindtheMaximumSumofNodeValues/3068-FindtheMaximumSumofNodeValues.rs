// Last updated: 23.05.2025, 09:31:15
impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _: Vec<Vec<i32>>) -> i64 {
        let mut nums = nums
            .into_iter()
            .map(|num| (num, num ^ k))
            .collect::<Vec<_>>();
        nums.sort_unstable_by_key(|(num, changed)| num - changed);
        let mut profitable_count = nums.partition_point(|(num, changed)| num < changed);
        if profitable_count & 1 == 1 {
            let profitless = nums.get(profitable_count).copied().unwrap_or_default();
            let profitable = nums.get(profitable_count - 1).copied().unwrap_or_default();
            if profitable_count < nums.len()
                && profitless.0 - profitless.1 < profitable.1 - profitable.0
            {
                profitable_count += 1;
            } else {
                profitable_count -= 1;
            }
        }
        nums.iter()
            .take(profitable_count)
            .map(|(_, changed)| *changed as i64)
            .sum::<i64>()
            + nums
                .iter()
                .skip(profitable_count)
                .map(|(num, _)| *num as i64)
                .sum::<i64>()
    }
}