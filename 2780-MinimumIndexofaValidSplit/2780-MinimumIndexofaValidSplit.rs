// Last updated: 27.03.2025, 09:38:26
impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        let mut iter = nums.iter();
        let mut dominant = *iter.next().unwrap();
        let mut dominant_count = 1;
        let mut num_count: std::collections::HashMap<i32, usize> = std::collections::HashMap::with_capacity(nums.len().div_ceil(2));
        num_count.insert(dominant, 1);
        for &num in iter {
            *num_count.entry(num).or_default() += 1;
            if num == dominant {
                dominant_count += 1;
            } else {
                dominant_count -= 1;
                if dominant_count < 0 {
                    dominant = num;
                    dominant_count = 1;
                }
            }
        }
        if num_count[&dominant] * 2 - nums.len() < 2 {
            return -1;
        }
        let mut dominant_count = 0;
        for (index, &num) in nums.iter().enumerate() {
            if num == dominant {
                dominant_count += 1;
            }
            if (index + 1) / 2 < dominant_count  {
                return index as i32;
            }
        }
        -1
    }
}