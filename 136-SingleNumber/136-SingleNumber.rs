impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut bit_sum = 0;
        for num in nums {
            bit_sum ^= num;
        }
        bit_sum
    }
}