// Last updated: 17.07.2025, 13:24:55
impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut dp = vec![vec![0; k]; k];
        for num in nums {
            let cur_rem = num as usize % k;
            let (lower_dp, remaining) = dp.split_at_mut(cur_rem);
            let (cur_dp, higher_dp) = remaining.split_first_mut().unwrap();
            cur_dp[cur_rem] += 1;
            for (dest, source) in cur_dp.iter_mut().zip(lower_dp) {
                *dest = source[cur_rem] + 1;
            }
            for (dest, source) in cur_dp.iter_mut().skip(cur_rem + 1).zip(higher_dp) {
                *dest = source[cur_rem] + 1;
            }
        }
        dp.into_iter()
            .map(|row| row.into_iter().max().unwrap())
            .max()
            .unwrap()
    }
}