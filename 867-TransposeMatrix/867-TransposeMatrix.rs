// Last updated: 06.05.2025, 15:38:37
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (first, remaining) = matrix.split_first().unwrap();
        let mut result = Vec::with_capacity(first.len());
        for cell in first {
            let mut row = Vec::with_capacity(matrix.len());
            row.push(*cell);
            result.push(row);
        }
        for row in remaining {
            for (destination, source) in result.iter_mut().zip(row) {
                destination.push(*source);
            }
        }
        result
    }
}