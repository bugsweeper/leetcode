// Last updated: 03.06.2025, 12:11:05
impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        let mut distance = k;
        for num in nums {
            if num == 0 {
                distance += 1;
            } else {
                if distance < k {
                    return false;
                }
                distance = 0;
            }
        }
        true
    }
}