// Last updated: 15.05.2025, 22:23:29
impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let sum: i32 = arr.iter().sum();
        if sum % 3 != 0 {
            return false;
        }
        let part_sum = sum / 3;
        let mut sum = 0;
        let mut parts = 0;
        for num in arr {
            sum += num;
            if sum == part_sum {
                if parts > 1 {
                    return true;
                }
                parts += 1;
                sum = 0;
            }
        }
        false
    }
}