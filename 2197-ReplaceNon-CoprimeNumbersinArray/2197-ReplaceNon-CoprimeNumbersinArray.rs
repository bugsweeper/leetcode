// Last updated: 16.09.2025, 23:25:34
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
        let mut cur = nums[0];
        let mut answer = Vec::with_capacity(nums.len());
        for num in nums.into_iter().skip(1).chain(std::iter::once(1)) {
            let gcd1 = gcd(cur, num);
            if gcd1 == 1 {
                // check if last item could be merged with cur
                while let Some(prev) = answer.pop() {
                    let gcd2 = gcd(prev, cur);
                    if gcd2 == 1 {
                        // at least we tried
                        answer.push(prev);
                        break;
                    } else {
                        cur = cur / gcd2 * prev;
                    }
                }
                answer.push(cur);
                cur = num;
            } else {
                cur = cur / gcd1 * num;
            }
        }
        answer
    }
}