// Last updated: 25.04.2025, 14:49:30
use std::collections::VecDeque;
use std::iter::once;

impl Solution {
    pub fn count_interesting_subarrays(nums: Vec<i32>, modulo: i32, k: i32) -> i64 {
        let n = nums.len();
        let modulo_usize = modulo as usize;
        let mut max_queue_size = if k == 0 {
            modulo_usize
        } else {
            k as usize
        };
        if max_queue_size > n {
            max_queue_size = 0;
            if k > 0 {
                return 0;
            }
        }
        let mut distances = VecDeque::with_capacity(max_queue_size);
        let mut accumulated_starting_points =
            vec![0; if modulo_usize < n { modulo_usize } else { 0 }];
        let mut starting_point_index = 0;
        let mut start_index = 0;
        let mut subarrays_count = 0;
        for (index, num) in nums.into_iter().chain(once(k)).enumerate() {
            if num % modulo == k {
                let new_distance = (index - start_index + 1) as i64;
                if k == 0 {
                    subarrays_count += new_distance * (new_distance - 1) / 2;
                }
                if max_queue_size != 0 {
                    if distances.len() == max_queue_size {
                        let old_distance = distances.pop_front().unwrap();
                        if modulo_usize < n {
                            let starting_points = accumulated_starting_points
                                .get_mut(starting_point_index)
                                .unwrap();
                            *starting_points += old_distance;
                            subarrays_count += *starting_points * new_distance;
                            starting_point_index = (starting_point_index + 1) % modulo_usize;
                        } else {
                            subarrays_count += old_distance * new_distance;
                        }
                    }
                    distances.push_back(new_distance);
                }
                start_index = index + 1;
            }
        }
        subarrays_count
    }
}