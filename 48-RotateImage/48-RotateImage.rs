impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // n here is not size, but last index
        let n = matrix.len() - 1;
        for i1 in 0..matrix.len() / 2 {
            for j1 in i1..n - i1 {
                let mut k = *unsafe { matrix.get_unchecked(i1).get_unchecked(j1) };
                for (i2, j2) in [(j1, n - i1), (n - i1, n - j1), (n - j1, i1), (i1, j1)] {
                    std::mem::swap(&mut k, unsafe {
                        matrix.get_unchecked_mut(i2).get_unchecked_mut(j2)
                    });
                }
            }
        }
    }
}