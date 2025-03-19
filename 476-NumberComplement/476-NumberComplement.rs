impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        (-1-num) & !(-(1 << i32::BITS - num.leading_zeros()))
    }
}