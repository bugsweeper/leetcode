impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut result = Vec::with_capacity(num_rows);
        result.push(vec![1]);
        for i in 1..num_rows {
            let mut new_row = Vec::with_capacity(i + 1);
            new_row.push(1);
            let prev_row = result.get(i - 1).unwrap();
            for j in 1..i {
                new_row.push(prev_row[j-1] + prev_row[j]);
            }
            new_row.push(1);
            result.push(new_row);
        }
        result
    }
}