// Last updated: 19.05.2025, 12:01:02
impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut expected = heights.clone();
        expected.sort_unstable();
        expected
            .into_iter()
            .zip(heights)
            .filter(|(expected, height)| expected != height)
            .count() as i32
    }
}