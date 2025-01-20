impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let mut value2index = std::collections::HashMap::with_capacity(arr.len());
        for (i, row) in mat.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                value2index.insert(value, (i, j));
            }
        }
        let m = mat.len();
        let n = arr.len() / m;
        let mut painted_in_row = vec![0; m];
        let mut painted_in_column = vec![0; n];
        for (index, value) in arr.into_iter().enumerate() {
            let (i, j) = value2index[&value];
            let painted_in_row = unsafe { painted_in_row.get_unchecked_mut(i) };
            *painted_in_row += 1;
            if *painted_in_row == n {
                return index as i32;
            }
            let painted_in_column = unsafe { painted_in_column.get_unchecked_mut(j) };
            *painted_in_column += 1;
            if *painted_in_column == m {
                return index as i32;
            }
        }
        (m * n) as i32
    }
}