impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let m = matrix.len();
        let n = unsafe { matrix.get_unchecked(0) }.len();
        let mut answer = Vec::with_capacity(m * n);
        // Safety: bounds always less than `m` and `n`, skipping bounds checks
        let (mut left, mut right, mut top, mut bottom) = (0, n - 1, 0, m - 1);
        loop {
            if left <= right {
                answer.extend_from_slice(unsafe { &matrix.get_unchecked(top)[left..=right] });
                top += 1;
            } else {
                break;
            }
            if top <= bottom {
                for row in &matrix[top..=bottom] {
                    answer.push(*unsafe { row.get_unchecked(right) });
                }
                if right == 0 {
                    break;
                }
                right -= 1;
            } else {
                break;
            }
            if left <= right {
                answer.extend(
                    unsafe { matrix.get_unchecked(bottom) }[left..=right]
                        .iter()
                        .rev(),
                );
                if bottom == 0 {
                    break;
                }
                bottom -= 1;
            } else {
                break;
            }
            if top <= bottom {
                for row in matrix[top..=bottom].iter().rev() {
                    answer.push(*unsafe { row.get_unchecked(left) });
                }
                left += 1;
            } else {
                break;
            }
        }
        answer
    }
}