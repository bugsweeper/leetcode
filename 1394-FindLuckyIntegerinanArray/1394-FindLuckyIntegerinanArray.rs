// Last updated: 02.06.2025, 16:45:21
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let max_candies = *candies.iter().max().unwrap();
        let min_to_have = max_candies - extra_candies;
        candies.into_iter().map(|candies| candies >= min_to_have).collect()
    }
}