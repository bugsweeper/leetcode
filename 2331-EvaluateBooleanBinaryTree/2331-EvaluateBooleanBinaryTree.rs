// Last updated: 08.09.2025, 14:55:37
impl Solution {
    pub fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; 101];
        for &num in &nums {
            count[num as usize] += 1;
        }
        let mut leftover = 0;
        for count in count {
            if count & 1 == 1 {
                leftover += 1;
            }
        }
        vec![(nums.len() as i32 - leftover) >> 1, leftover]
    }
}