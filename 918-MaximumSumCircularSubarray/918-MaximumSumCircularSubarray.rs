// Last updated: 03.04.2025, 11:23:57
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut local_min_sum = 0;
        let mut min_sum = i32::MAX;
        let mut local_max_sum = 0;
        let mut max_sum = i32::MIN;
        for num in nums {
            local_max_sum = local_max_sum.max(0) + num;
            local_min_sum = local_min_sum.min(0) + num;
            sum += num;
            min_sum = min_sum.min(local_min_sum);
            max_sum = max_sum.max(local_max_sum);
        }
        if sum == min_sum {
            max_sum
        } else {
            max_sum.max(sum - min_sum)
        }
    }
}