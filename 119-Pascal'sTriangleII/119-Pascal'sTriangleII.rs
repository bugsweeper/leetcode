impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let n = row_index as usize;
        if row_index == 0 {
            return vec![1];
        }
        let mut result = Vec::with_capacity(n + 1);
        result.push(1);
        let mut value = 1;
        for i in 0..n {
            value = value * (n - i) / (i + 1);
            result.push(value as i32);
        }
        result
    }
}