// Last updated: 16.06.2025, 10:44:59
impl Solution {
    pub fn get_maximum_generated(n: i32) -> i32 {
        if n < 2 {
            return n;
        }
        let n = n as usize;
        let mut nums = Vec::with_capacity(n + 1);
        nums.push(0);
        nums.push(1);
        let mut max = 1;
        for i in 2..=n {
            let num = if i & 1 == 0 {
                nums[i >> 1]
            } else {
                let i = i >> 1;
                nums[i] + nums[i + 1]
            };
            max = max.max(num);
            nums.push(num);
        }
        max
    }
}