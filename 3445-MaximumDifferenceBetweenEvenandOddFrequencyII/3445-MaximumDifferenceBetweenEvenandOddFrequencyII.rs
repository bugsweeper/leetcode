// Last updated: 11.06.2025, 15:07:43
// from '0' to '4'
const DIGITS: usize = 5;

// using only 4 combinations for a and b parity combinations
// other 4 combinations is backup for b - 2 values
const PARITY_COMBINATIONS: usize = 8;

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        let mut prefix_sum: [Vec<i32>; DIGITS] =
            std::array::from_fn(|_| Vec::with_capacity(s.len()));
        for prefix_sum in &mut prefix_sum {
            prefix_sum.push(0);
        }
        let mut current_count = [0; DIGITS];
        for byte in s.bytes() {
            current_count[(byte - b'0') as usize] += 1;
            for (&count, prefix_sum) in current_count.iter().zip(prefix_sum.iter_mut()) {
                prefix_sum.push(count);
            }
        }
        // greedy moment: valid answer could be only if even number is at least 2 and odd number is at least 1
        // by sorting, we are trying pairs in descending order of possible max difference
        // this is why we can skip more pairs which can't improve previous best difference
        let mut pairs = Vec::with_capacity(DIGITS * (DIGITS - 1));
        for (i, &count_a) in current_count.iter().enumerate() {
            for (j, &count_b) in current_count.iter().enumerate() {
                if i == j {
                    continue;
                }
                let count_a = if count_a & 1 == 0 {
                    count_a - 1
                } else {
                    count_a
                };
                if count_a == 0 {
                    continue;
                }
                let count_b = if count_b & 1 == 0 {
                    count_b
                } else {
                    count_b - 1
                };
                if count_b < 2 {
                    continue;
                }
                pairs.push((count_a, i, count_b, j));
            }
        }
        pairs.sort_unstable_by_key(|&(count_a, _, count_b, _)| count_b - count_a);
        let mut max_diff = i32::MIN;
        let k = k as usize;
        for (count_a, a_i, _, b_i) in pairs {
            if count_a - 2 <= max_diff {
                // best case will not improve max_diff, skipping this pair
                continue;
            }
            let prefix_a = &prefix_sum[a_i];
            let prefix_b = &prefix_sum[b_i];
            let mut min_prev_diff = [i32::MAX; PARITY_COMBINATIONS];
            let mut saved_prev_b = 0;
            for ((&prev_a, &cur_a), (&prev_b, &cur_b)) in prefix_a
                .iter()
                .zip(prefix_a.iter().skip(k))
                .zip(prefix_b.iter().zip(prefix_b.iter().skip(k)))
            {
                let prev_index = (((prev_a & 1) << 1) + (prev_b & 1)) as usize;
                if prev_b != saved_prev_b {
                    // need to make backup
                    min_prev_diff[prev_index | 4] = min_prev_diff[prev_index];
                    let other_a_index = prev_index ^ 2;
                    min_prev_diff[other_a_index | 4] = min_prev_diff[other_a_index];
                    saved_prev_b = prev_b;
                }
                let prev_diff = &mut min_prev_diff[prev_index];
                *prev_diff = (*prev_diff).min(prev_a - prev_b);
                let cur_index = ((cur_a & 1) << 1) + (cur_b & 1);
                let mut cur_inverse_index = (cur_index ^ 2) as usize;
                if cur_b == prev_b {
                    // invalid case, go to backup
                    cur_inverse_index |= 4;
                }
                let prev_inverse_diff = min_prev_diff[cur_inverse_index];
                if prev_inverse_diff != i32::MAX {
                    max_diff = max_diff.max(cur_a - cur_b - min_prev_diff[cur_inverse_index]);
                }
            }
        }
        max_diff
    }
}