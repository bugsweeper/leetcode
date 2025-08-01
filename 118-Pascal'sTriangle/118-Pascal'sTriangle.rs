// Last updated: 01.08.2025, 09:35:59
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut result = Vec::with_capacity(num_rows);
        result.push(vec![1]);
        for i in 1..num_rows {
            let mut new_row = Vec::with_capacity(i + 1);
            let mut prev_num1 = 0;
            for &prev_num2 in result.get(i - 1).unwrap() {
                new_row.push(prev_num1 + prev_num2);
                prev_num1 = prev_num2;
            }
            new_row.push(1);
            result.push(new_row);
        }
        result
    }
}