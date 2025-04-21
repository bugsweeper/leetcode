// Last updated: 21.04.2025, 09:13:43
impl Solution {
    pub fn number_of_arrays(differences: Vec<i32>, lower: i32, upper: i32) -> i32 {
        let mut sum = 0;
        let mut min = 0;
        let mut max = 0;
        for diff in differences {
            sum += diff as i64;
            min = min.min(sum);
            max = max.max(sum);
            if max - min > (upper - lower) as i64 {
                return 0;
            }
        }
        ((upper - lower) - (max - min) as i32 + 1).max(0)
    }
}