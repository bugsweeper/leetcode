// Last updated: 11.05.2025, 12:42:10
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut odd_count = 0;
        for num in arr {
            if num & 1 == 1 {
                odd_count += 1;
                if odd_count == 3 {
                    return true;
                }
            } else {
                odd_count = 0;
            }
        }
        false
    }
}