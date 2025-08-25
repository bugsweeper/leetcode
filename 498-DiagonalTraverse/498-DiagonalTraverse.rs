// Last updated: 25.08.2025, 10:18:41
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (mat.len(), mat[0].len());
        let (last_i, last_j) = (m - 1, n - 1);
        let mut diagonal = Vec::with_capacity(m * n);
        for diagonal_index in 0..m + n - 1 {
            let start = diagonal_index.saturating_sub(last_j);
            let end = diagonal_index.min(last_i);
            if diagonal_index & 1 == 0 {
                for i in (start..=end).rev() {
                    diagonal.push(mat[i][diagonal_index - i]);
                }
            } else {
                for i in start..=end {
                    diagonal.push(mat[i][diagonal_index - i]);
                }
            }
        }
        diagonal
    }
}