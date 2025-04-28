// Last updated: 28.04.2025, 13:55:08
impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let (mut prev, mut matrix) = matrix.split_first().unwrap();
        while let Some((cur, remaining)) = matrix.split_first() {
            if prev.iter().zip(cur.iter().skip(1)).any(|(prev, cur)| prev != cur) {
                return false;
            }
            prev = cur;
            matrix = remaining;
        }
        true
    }
}