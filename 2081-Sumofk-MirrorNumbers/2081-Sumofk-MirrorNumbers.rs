// Last updated: 23.06.2025, 14:25:05
struct Mirror {
    base: i64,
    odd_digits: bool,
    left: i64,
    middle_digit: i64,
    left_limit: i64,
}

impl Mirror {
    fn new(base: i32) -> Self {
        let base = base as i64;
        Self {
            base,
            odd_digits: true,
            left: 0,
            middle_digit: 0,
            left_limit: 1,
        }
    }
}

impl Iterator for Mirror {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        // change components for next number
        if self.odd_digits {
            self.middle_digit += 1;
            if self.middle_digit == self.base {
                self.middle_digit = 0;
                self.left += 1;
                if self.left == self.left_limit {
                    self.odd_digits = false;
                    self.left_limit *= self.base;
                }
            }
        } else {
            self.left += 1;
            if self.left == self.left_limit {
                self.left /= self.base;
                self.odd_digits = true;
            }
        }
        // generate number from components
        if self.odd_digits {
            let mut left = self.left;
            let mut output = left * self.base + self.middle_digit;
            while left > 0 {
                output = output * self.base + left % self.base;
                left /= self.base;
            }
            Some(output)
        } else {
            let mut left = self.left;
            let mut output = left;
            while left > 0 {
                output = output * self.base + left % self.base;
                left /= self.base;
            }
            Some(output)
        }
    }
}

#[inline(always)]
fn is_10_mirror(mut may_be_mirror: i64) -> bool {
    let mut digits = Vec::with_capacity(11);
    while may_be_mirror > 0 {
        digits.push((may_be_mirror % 10) as u8);
        may_be_mirror /= 10;
    }
    for (&left, &right) in digits
        .iter()
        .zip(digits.iter().rev())
        .take(digits.len() / 2)
    {
        if left != right {
            return false;
        }
    }
    true
}

impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        let base_k = Mirror::new(k);
        let mut sum = 0;
        let mut count = 0;
        for mirror in base_k {
            if is_10_mirror(mirror) {
                count += 1;
                sum += mirror;
                if count == n {
                    break;
                }
            }
        }
        sum
    }
}