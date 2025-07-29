// Last updated: 29.07.2025, 14:58:24
impl Solution {
    pub fn max_distance(colors: Vec<i32>) -> i32 {
        (if colors[0] == *colors.last().unwrap() {
            let same = colors[0];
            let left = colors.iter().position(|&color| color != same).unwrap();
            let right = colors.iter().rev().position(|&color| color != same).unwrap();
            colors.len() - left.min(right)
        } else {
            colors.len()
        }) as i32 - 1
    }
}