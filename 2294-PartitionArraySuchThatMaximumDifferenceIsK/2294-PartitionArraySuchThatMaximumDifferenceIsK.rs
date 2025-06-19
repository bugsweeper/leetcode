// Last updated: 19.06.2025, 09:23:01
impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut first = *nums.first().unwrap();
        let mut partitions = 1;
        for num in nums.into_iter().skip(1) {
            if num - first > k {
                first = num;
                partitions += 1;
            }
        }
        partitions
    }
}