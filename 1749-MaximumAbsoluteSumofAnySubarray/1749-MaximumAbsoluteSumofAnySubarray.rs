impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut sum = 0;
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        // convert vec of values to vec of prefix sums
        for num in nums {
            sum += num;
            min = min.min(sum);
            max = max.max(sum);
        }
        max.abs().max(min.abs()).max((max-min).abs())
    }
}