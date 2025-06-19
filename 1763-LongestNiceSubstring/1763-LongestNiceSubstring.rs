// Last updated: 19.06.2025, 15:43:40
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut operations = 0;
        let mut expected = 1;
        for num in nums {
            if num < expected {
                operations += expected - num;
                expected += 1;
            } else {
                expected = num + 1;
            }
        }
        operations
    }
}