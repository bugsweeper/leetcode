// Last updated: 21.05.2025, 16:45:21
impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut columns = vec![0; n as usize];
        let mut rows = vec![0; m as usize];
        for point in indices {
            let [i, j] = point[..] else {
                unimplemented!();
            };
            columns[j as usize] += 1;
            rows[i as usize] += 1;
        }
        let (mut odd, mut even) = (0, 0);
        for column in columns {
            if column & 1 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        let mut odd_cells = 0;
        for row in rows {
            odd_cells += if row & 1 == 0 { odd } else { even };
        }
        odd_cells
    }
}