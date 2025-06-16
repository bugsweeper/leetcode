// Last updated: 16.06.2025, 12:57:09
impl Solution {
    pub fn number_of_matches(n: i32) -> i32 {
        let mut n = n;
        let mut total_matches = 0;
        while n > 1 {
            let matches = n >> 1;
            total_matches += matches;
            n = matches + (n & 1);
        }
        total_matches
    }
}