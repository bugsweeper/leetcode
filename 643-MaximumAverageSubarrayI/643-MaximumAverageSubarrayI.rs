// Last updated: 14.04.2025, 11:46:24
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut iter = nums.iter();
        let mut sum = 0;
        for _ in 0..k {
            sum += *iter.next().unwrap();
        }
        let mut max_sum = sum;
        for (&prev, &next) in nums.iter().zip(iter) {
            sum += next - prev;
            max_sum = max_sum.max(sum);
        }
        max_sum as f64 / k as f64
    }
}