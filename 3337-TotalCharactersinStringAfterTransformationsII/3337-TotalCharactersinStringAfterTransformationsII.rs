// Last updated: 14.05.2025, 12:18:04
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;
const MODULO: i64 = 1_000_000_007;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32, nums: Vec<i32>) -> i32 {
        let mut matrix = vec![vec![0; ABC_LEN]; ABC_LEN];
        for (index, num) in nums.into_iter().enumerate() {
            for row in matrix.iter_mut().skip(index + 1).take(num as usize) {
                row[index] = 1;
            }
            if index + 1 + num as usize > ABC_LEN {
                for row in matrix.iter_mut().take(num as usize + index + 1 - ABC_LEN) {
                    row[index] = 1;
                }
            }
        }
        let mut frequency = vec![0; ABC_LEN];
        for &byte in s.as_bytes() {
            frequency[(byte - b'a') as usize] += 1;
        }
        let mut t = t;
        while t > 0 {
            if t & 1 == 1 {
                let mut vector_product = Vec::with_capacity(ABC_LEN);
                for row in &matrix {
                    let mut count = 0;
                    for (&frequency, &multiplier) in frequency.iter().zip(row) {
                        count = (count + frequency as i64 * multiplier as i64) % MODULO
                    }
                    vector_product.push(count)
                }
                std::mem::swap(&mut frequency, &mut vector_product);
            }
            t >>= 1;
            if t > 0 {
                let mut result_matrix = Vec::with_capacity(ABC_LEN);
                // following chain a -> b -> c, we need determine can we go from a to c via b
                for b2c_row in &matrix {
                    let mut a2c_row = vec![0; ABC_LEN];
                    for (b2c_cell, a2b_row) in b2c_row.iter().zip(matrix.iter()) {
                        for (&a2b_cell, a2c_cell) in a2b_row.iter().zip(a2c_row.iter_mut()) {
                            *a2c_cell = (*a2c_cell + a2b_cell * b2c_cell) % MODULO;
                        }
                    }
                    result_matrix.push(a2c_row);
                }
                std::mem::swap(&mut matrix, &mut result_matrix);
            }
        }
        (frequency.into_iter().sum::<i64>() % MODULO) as i32
    }
}