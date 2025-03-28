// Last updated: 28.03.2025, 14:28:46
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let last = *nums.last().unwrap();
        let index_of_min = nums.partition_point(|&num| num > last);
        if index_of_min == nums.len() {
            *nums.first().unwrap()
        } else {
            nums[index_of_min]
        }
    }
}