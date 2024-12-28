impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        use std::collections::VecDeque;
        let mut cache1: VecDeque<(usize, i32)> = VecDeque::with_capacity(k+1);
        let mut cache2: VecDeque<(usize, usize, i32)> = VecDeque::with_capacity(k+1);
        let (mut result_index1, mut result_index2, mut result_index3, mut result_sum) = (0, 0, 0, i32::MIN);
        let mut current_sum = nums.iter().take(k).sum();
        cache1.push_back((0, current_sum));
        let mut current_num_index = k;
        while current_num_index < nums.len() {
            let (prev_index1, prev_sum1) = cache1.back().unwrap();
            current_sum = current_sum - nums[current_num_index - k] + nums[current_num_index];
            let current_index = current_num_index - k + 1;
            cache1.push_back(if current_sum > *prev_sum1 {
                (current_index, current_sum)
            } else {
                (*prev_index1, *prev_sum1)
            });
            if cache1.len() == k + 1 {
                let (first_index1, first_sum1) = cache1.pop_front().unwrap();
                let current_sum2 = first_sum1 + current_sum;
                cache2.push_back(if let Some((prev_index1, prev_index2, prev_sum2)) = cache2.back() {
                    if current_sum2 > *prev_sum2 {
                        (first_index1, current_index, current_sum2)
                    } else {
                        (*prev_index1, *prev_index2, *prev_sum2)
                    }
                } else {
                    (first_index1, current_index, current_sum2)
                });
                if cache2.len() == k + 1 {
                    let (first_index1, first_index2, first_sum2) = cache2.pop_front().unwrap();
                    let current_sum3 = first_sum2 + current_sum;
                    if current_sum3 > result_sum {
                        result_sum = current_sum3;
                        result_index1 = first_index1;
                        result_index2 = first_index2;
                        result_index3 = current_index;
                    }
                }
            }
            current_num_index += 1
        }

        vec![result_index1 as i32, result_index2 as i32, result_index3 as i32]
    }
}