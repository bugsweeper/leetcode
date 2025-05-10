// Last updated: 10.05.2025, 14:10:14
impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let (sum1, zeros1) = nums1.into_iter().fold((0, 0), |(sum, zeros), num| {
            if num == 0 {
                (sum, zeros + 1)
            } else {
                (sum + num as i64, zeros)
            }
        });
        let (sum2, zeros2) = nums2.into_iter().fold((0, 0), |(sum, zeros), num| {
            if num == 0 {
                (sum, zeros + 1)
            } else {
                (sum + num as i64, zeros)
            }
        });
        match (zeros1, zeros2) {
            (0, 0) => {
                if sum1 == sum2 {
                    sum1
                } else {
                    -1
                }
            }
            (0, _) => {
                if sum1 >= sum2 + zeros2 {
                    sum1
                } else {
                    -1
                }
            }
            (_, 0) => {
                if sum2 >= sum1 + zeros1 {
                    sum2
                } else {
                    -1
                }
            }
            _ => (sum1 + zeros1).max(sum2 + zeros2),
        }
    }
}