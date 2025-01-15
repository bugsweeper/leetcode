use std::cmp::Ordering;

impl Solution {
    pub fn minimize_xor(num1: i32, num2: i32) -> i32 {
        let mut num1 = num1;
        let mut num1_ones = num1.count_ones();
        let num2_ones = num2.count_ones();
        match num1_ones.cmp(&num2_ones) {
            Ordering::Equal => {}
            Ordering::Greater => {
                for i in 0..i32::BITS {
                    if num1 & (1 << i) != 0 {
                        num1 &= !(1 << i);
                        num1_ones -= 1;
                        if num1_ones == num2_ones {
                            break;
                        }
                    }
                }
            }
            Ordering::Less => {
                for i in 0..i32::BITS {
                    if num1 & (1 << i) == 0 {
                        num1 |= 1 << i;
                        num1_ones += 1;
                        if num1_ones == num2_ones {
                            break;
                        }
                    }
                }
            }
        }
        num1
    }
}