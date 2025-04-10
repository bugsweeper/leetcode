// Last updated: 10.04.2025, 13:33:37
impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        let mut result = left & right;
        if result == 0 {
            return 0;
        }
        let mut left = left;
        let mut bit = 1;
        for _ in 0..i32::BITS {
            if left & bit != 0 {
                left = left.saturating_add(bit);
            }
            if left >= right {
                return result;
            }
            result &= left;
            bit <<= 1;
        }
        result
    }
}