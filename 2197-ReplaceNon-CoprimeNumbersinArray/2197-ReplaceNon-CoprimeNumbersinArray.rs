// Last updated: 16.09.2025, 23:33:50
#[inline]
fn gcd(mut x: i32, mut y: i32) -> i32 {
    if x < y {
        (x, y) = (y, x)
    }
    while y > 0 {
        (x, y) = (y, x % y);
    }
    x
}

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut answer = Vec::with_capacity(nums.len());
        for mut num in nums {
            while let Some(&last) = answer.last() {
                let gcd = gcd(last, num);
                if gcd == 1 {
                    break;
                } else {
                    num *= last / gcd;
                    answer.pop();
                }
            }
            answer.push(num);
        }
        answer
    }
}