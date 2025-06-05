// Last updated: 05.06.2025, 23:37:42
// O(1) Solution based on counting each `num` usage count and arithmetic progressions
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        if arr.len() < 3 {
            return arr.iter().sum();
        }
        let mut sum = 0;
        let max_len = if arr.len() & 1 == 1 {
            arr.len()
        } else {
            arr.len() - 1
        };
        let subarrays_count = (max_len + 1) >> 1;
        let max_right = arr.len() - 1;
        for (left, &num) in arr.iter().enumerate() {
            let right = max_right - left;
            let max_shift = left.min(right);
            if max_shift == 0 {
                sum += num * subarrays_count as i32;
                continue;
            }
            // if iterate length of subarray between 1 and max_len, then
            // amount of `num` usage in all subarrays of this length is minimum between
            //  1) len; because num can take only 'length of subarray' positions inside subarray (low len is limitation for amount of `num`)
            // for example is subarray has length = 1, then there is only one such subarray, that contains this `num`
            //  2) max_shift + 1; because max amount of places of subarray is limited by closest distance between num and edge of array (low max_shift is limitation)
            // for example if `num`` is placed at position 0 then there is only one subarray of each length, which contains this `num`
            //  3) arr.len() - len + 1; because if subarray is large, then amount of it places limited by edges of array (huge len is limitation)
            // for example if array has length = 100 and subarray has length = 99 then there are only two positions for subarray
            // starting from 0 and 1, even if `num`` is placed at the middle of array
            // so there are three ranges for formulas of amount of `num` usage:
            //  |1  |________|3  |
            //  |  /|2       |\  |
            //  | / |        | \ |
            //  |/  |        |  \|
            let (max_shift_odd, max_shift_even) = if max_shift & 1 == 1 {
                (max_shift, max_shift + 1)
            } else {
                (max_shift - 1, max_shift)
            };
            let first_range_item_count = ((max_shift_odd + 1) >> 1).min(subarrays_count >> 1);
            let (third_range_low, third_range_high) = if arr.len() & 1 == 0 {
                (2, max_shift_even)
            } else {
                (1, max_shift_odd)
            };
            let third_range_item_count = ((third_range_high - third_range_low + 2) >> 1).min(subarrays_count >> 1);
            let first_range_sum = ((1 + max_shift_odd) * first_range_item_count) >> 1;
            let third_range_sum =
                ((third_range_low + third_range_high) * third_range_item_count) >> 1;
            let second_range_item_count =
                subarrays_count - first_range_item_count - third_range_item_count;
            let second_range_sum = (max_shift + 1) * second_range_item_count;
            sum += num * (first_range_sum + second_range_sum + third_range_sum) as i32;
            // alternative more simple way to calculate is:
            // for len in (1..=max_len).step_by(2) {
            //     let count = (max_shift.min(arr.len() - len) + 1).min(len);
            //     sum += count as i32 * num;
            // }
            // but it is not O(1)
        }
        sum
    }
}