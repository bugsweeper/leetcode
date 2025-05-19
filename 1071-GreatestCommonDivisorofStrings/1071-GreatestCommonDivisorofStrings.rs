// Last updated: 19.05.2025, 15:08:00
impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut zeros = Vec::with_capacity(arr.len() / 2);
        for (index, &num) in arr.iter().enumerate() {
            if num == 0 {
                if index + zeros.len() >= arr.len() - 1 {
                    break;
                }
                zeros.push(index);
            }
        }
        let mut end = arr.len() - zeros.len();
        for (prev_count, zero_index) in zeros.into_iter().enumerate().rev() {
            arr.copy_within(zero_index..end, zero_index + prev_count + 1);
            arr[zero_index + prev_count] = 0;
            end = zero_index;
        }
    }
}