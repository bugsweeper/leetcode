// Last updated: 15.04.2025, 22:15:20
#[inline]
fn num2index(nums: &[i32]) -> Vec<usize> {
    let mut result = vec![0; nums.len()];
    for (index, &num) in nums.iter().enumerate() {
        result[num as usize] = index;
    }
    result
}

/// Fenwick Tree (BIT - Binary Indexed Tree) getter
#[inline]
fn get_prefix_count(bit: &mut [i64], mut index: i32) -> i64 {
    index += 1;
    let mut sum = 0;
    while index > 0 {
        sum += bit[index as usize];
        index -= index & -index;
    }
    sum
}

/// Fenwick Tree (BIT - Binary Indexed Tree) getter
#[inline]
fn increment(bit: &mut [i64], mut index: i32) {
    let n = bit.len() as i32;
    index += 1;
    while index < n {
        bit[index as usize] += 1;
        index += index & -index;
    }
}

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();
        let end = n - 1;
        let indexes2 = num2index(&nums2);
        let mut left_bit = vec![0; n + 1];
        let mut left = vec![0; n];
        for &num in &nums1 {
            let num = num as usize;
            let index2 = indexes2[num] as i32;
            if index2 > 0 {
                left[num] = get_prefix_count(&mut left_bit, index2 - 1);
            }
            increment(&mut left_bit, index2);
        }
        let mut right_bit = vec![0; n + 1];
        let mut count = 0;
        for &num in nums1.iter().rev() {
            let num = num as usize;
            let index2 = (end - indexes2[num]) as i32;
            if index2 > 0 {
                let right = get_prefix_count(&mut right_bit, index2 - 1);
                count += right * left[num];
            }
            increment(&mut right_bit, index2);
        }
        count
    }
}
