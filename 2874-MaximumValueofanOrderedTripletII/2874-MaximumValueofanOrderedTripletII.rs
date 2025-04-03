// Last updated: 03.04.2025, 09:51:53
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut suffix_max = vec![0; nums.len()];
        let mut max = 0;
        for (num, suffix_max) in nums.iter().zip(suffix_max.iter_mut()).rev() {
            *suffix_max = max;
            max = max.max(*num);
        }
        let mut max = 0;
        nums.into_iter()
            .zip(suffix_max)
            .fold(0, |mut value, (num, suffix)| {
                let num = num as i64;
                value = value.max((max - num) * suffix as i64);
                max = max.max(num);
                value
            })
    }
}