// Last updated: 21.05.2025, 00:02:59
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // finding median of two sorted arrays using binary search and divide and conquer algorithm
        let size = nums1.len() + nums2.len();
        let half = size / 2;
        let end1 = nums1.len() - 1;
        let end2 = nums2.len() - 1;
        // Trivial cases if one array is empty or if other array contains lower half of both arrays
        if nums1.is_empty() || nums2.len() > half && nums2[half] < nums1[0] {
            return (nums2[(size - 1) / 2] + nums2[half]) as f64 / 2.0;
        }
        if nums2.is_empty() || nums1.len() > half && nums1[half] < nums2[0] {
            return (nums1[(size - 1) / 2] + nums1[half]) as f64 / 2.0;
        }
        // finding the median of two sorted arrays using binary search and divide and conquer algorithm
        let mut left_count1 = 0;
        let mut left_count2 = 0;
        while half - (left_count1 + left_count2) > 1 {
            let remaining: usize = half - (left_count1 + left_count2);
            let step = remaining / 2;
            let middle1 = left_count1 + step;
            let middle2 = left_count2 + step;
            // if middle is out of bounds, median could be in other array
            if middle1 > end1 {
                if nums1[end1] <= nums2[middle2] {
                    left_count2 = half - nums1.len();
                    if size & 1 == 0 {
                        if left_count2 > 0 {
                            return (nums2[left_count2 - 1].max(nums1[end1]) + nums2[left_count2]) as f64 / 2.0;
                        } else {
                            return (nums1[end1] + nums2[0]) as f64 / 2.0;
                        }
                    } else {
                        return nums2[left_count2] as f64;
                    }
                } else {
                    left_count2 = middle2;
                }
                continue;
            }
            if middle2 > end2 {
                if nums2[end2] <= nums1[middle1] {
                    left_count1 = half - nums2.len();
                    if size & 1 == 0 {
                        if left_count1 > 0 {
                            return (nums1[left_count1 - 1].max(nums2[end2]) + nums1[left_count1]) as f64 / 2.0;
                        } else {
                            return (nums2[end2] + nums1[0]) as f64 / 2.0;
                        }
                    } else {
                        return nums1[left_count1] as f64;
                    }
                } else {
                    left_count1 = middle1;
                }
                continue;
            }
            if nums1[middle1 - 1] <= nums2[middle2] {
                left_count1 = middle1;
            }
            if nums2[middle2 - 1] <= nums1[middle1] {
                left_count2 = middle2;
            }
        }
        if half - (left_count1 + left_count2) > 0 {
            if nums1[left_count1] < nums2[left_count2] {
                left_count1 += 1;
            } else {
                left_count2 += 1;
            }
        }
        if size & 1 == 0 {
            (if left_count1 == 0 {
                nums2[left_count2 - 1]
            } else if left_count2 == 0 {
                nums1[left_count1 - 1]
            } else {
                nums1[left_count1 - 1].max(nums2[left_count2 - 1])
            } + nums1
                .get(left_count1)
                .copied()
                .unwrap_or(i32::MAX)
                .min(nums2.get(left_count2).copied().unwrap_or(i32::MAX))) as f64
                / 2.0
        } else {
            nums1
                .get(left_count1)
                .copied()
                .unwrap_or(i32::MAX)
                .min(nums2.get(left_count2).copied().unwrap_or(i32::MAX)) as f64
        }
    }
}