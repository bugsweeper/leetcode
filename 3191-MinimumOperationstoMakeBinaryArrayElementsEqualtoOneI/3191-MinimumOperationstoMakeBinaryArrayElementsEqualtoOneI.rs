const FIRST_BIT: i32 = 0b100;
const THREE_BITS: i32 = 0b111;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut bits = (nums[0] << 1) | nums[1];
        let mut operations = 0;
        for bit in nums.into_iter().skip(2) {
            bits = (bits << 1) | bit;
            if bits & FIRST_BIT == 0 {
                operations += 1;
                bits = !bits
            }
        }
        if bits & THREE_BITS == THREE_BITS {
            operations
        } else {
            -1
        }
    }
}