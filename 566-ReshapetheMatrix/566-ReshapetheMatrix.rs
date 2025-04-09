// Last updated: 09.04.2025, 13:01:58
use std::cmp::Ordering;

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let m = mat.len();
        let n = mat[0].len();
        let r = r as usize;
        let c = c as usize;
        if m * n != r * c || m == r {
            return mat;
        }
        let mut new_mat = vec![vec![]; r];
        let mut iter = mat.into_iter();
        let mut source = iter.next().unwrap();
        let mut slice = &source[..];
        for row in &mut new_mat {
            while row.len() < c {
                let remaining = c - row.len();
                if slice.is_empty() {
                    source = iter.next().unwrap();
                    slice = &source[..];
                }
                match slice.len().cmp(&remaining) {
                    Ordering::Greater => {
                        row.extend_from_slice(&slice[..remaining]);
                        slice = &slice[remaining..];
                    }
                    _ => {
                        row.extend_from_slice(slice);
                        slice = &source[..0];
                    }
                }
            }
        }
        new_mat
    }
}