// Last updated: 09.04.2025, 11:20:17
// Here b1 and b2 are bit counting variables
// which together count each bit 0 -> 1 -> 2 -> 0...
// with this truth table:
// n   b1  b2  next_b1 next_b2
// 0   0   0   0       0
// 0   1   0   1       0
// 0   0   1   0       1
// 1   0   0   1       0
// 1   1   0   0       1
// 1   0   1   0       0
// which can be described with following

// Perfect disjunctive normal form:
// b1_next = (!n & b1 & !b2) | (n & !b1 & !b2)
// b2_next = (!n & !b1 & b2) | (n & b1 & !b2)

// which can be simplified to:
// b1_next = !b2 & (n ^ b1),
// b2_next = b2 ^ n & b2 ^ b1

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let (mut b1, mut b2) = (0, 0);
        for num in nums {
            (b1, b2) = (!b2 & (num ^ b1), (b2 ^ num) & (b2 ^ b1));
        }
        // if every element appears three times exctpt one, which appears exactly once
        // then at the end b1 should contain that one element and b2 should be zero
        b1
    }
}