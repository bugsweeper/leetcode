// Last updated: 25.07.2025, 09:45:35
impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut seen: u128 = 0;
        let mut sum = 0;
        for &num in &nums {
            let mask = 1 << num;
            if num > 0 && seen & mask == 0 {
                seen |= mask;
                sum += num;
            }
        }
        if seen == 0 {
            nums.into_iter().max().unwrap()
        } else {
            sum
        }
    }
}