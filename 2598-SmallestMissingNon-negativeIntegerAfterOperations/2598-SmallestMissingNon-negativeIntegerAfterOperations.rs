// Last updated: 16.10.2025, 14:19:51
impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        if value == 1 {
            return nums.len() as i32;
        }
        let mut count = vec![0; value as usize];
        let mut min_count = nums.len();
        for num in nums {
            count[num.rem_euclid(value) as usize] += 1;
        }
        let mut min_modulo = value as usize;
        for (modulo_value, modulo_count) in count.into_iter().enumerate() {
            if modulo_count < min_count {
                min_count = modulo_count;
                min_modulo = modulo_value;
            }
        }
        (min_modulo + value as usize * min_count) as i32
    }
}