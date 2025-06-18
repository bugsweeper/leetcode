// Last updated: 18.06.2025, 09:37:21
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut should_insert_1 = true;
        for digit in digits.iter_mut().rev() {
            match *digit {
                9 => *digit = 0,
                _ => {
                    *digit += 1;
                    should_insert_1 = false;
                    break;
                }
            }
        }
        if should_insert_1 {
            digits.insert(0, 1);
        }
        digits
    }
}