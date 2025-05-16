// Last updated: 16.05.2025, 13:01:52
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        let end = mat.len() - 1;
        for (index, row) in mat.iter().enumerate() {
            sum += row[index] + row[end - index];
        }
        if mat.len() & 1 == 1 {
            let index = end / 2;
            sum -= mat[index][index];
        }
        sum
    }
}