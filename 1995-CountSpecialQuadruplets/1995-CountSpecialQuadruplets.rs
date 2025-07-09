// Last updated: 09.07.2025, 14:19:14
impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let (mut mins, mut maxes, mut min, mut max) = (
            vec![i32::MAX; nums.len()],
            vec![i32::MIN; nums.len()],
            i32::MAX,
            i32::MIN,
        );
        for ((cur_min, cur_max), &cur) in
            mins.iter_mut().zip(maxes.iter_mut()).zip(nums.iter()).rev()
        {
            min = min.min(cur);
            max = max.max(cur);
            *cur_min = min;
            *cur_max = max;
        }
        let mut count = 0;
        for (index_a, &a) in nums.iter().enumerate().take(nums.len() - 3) {
            let start_b = index_a + 1;
            if a + 2 * mins[start_b] > maxes[start_b] {
                continue;
            }
            for (index_b, &b) in nums.iter().enumerate().take(nums.len() - 2).skip(start_b) {
                let start_c = index_b + 1;
                let prefix = a + b;
                if prefix + mins[start_c] > maxes[start_c] {
                    continue;
                }
                // count each number mention
                let mut num_count = [0; 101];
                for &num in &nums[start_c..] {
                    num_count[num as usize] += 1;
                }
                for &c in nums.iter().take(nums.len() - 1).skip(start_c) {
                    if let Some(&num_count) = num_count.get((prefix + c) as usize) {
                        count += num_count;
                    }
                    num_count[c as usize] -= 1;
                }
            }
        }
        count
    }
}