// Last updated: 25.06.2025, 17:31:25
use std::cmp::Ordering;

impl Solution {
    pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
        let (min1, max1, min2, max2) = (
            nums1[0] as i64,
            *nums1.last().unwrap() as i64,
            nums2[0] as i64,
            *nums2.last().unwrap() as i64,
        );
        let (product1, product2, product3, product4) =
            (min1 * min2, min1 * max2, max1 * min2, max1 * max2);
        let min_product = product1.min(product2).min(product3).min(product4);
        let max_product = product1.max(product2).max(product3).max(product4);
        if k == 1 {
            return min_product;
        }
        let k = k as usize;
        if k == nums1.len() * nums2.len() {
            return max_product;
        }
        let mut product_base = min_product;
        let mut product_size = max_product - min_product;
        let mut nums1 = nums1;
        let mut nums2 = nums2;
        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }
        let negatives1 = nums1.partition_point(|&num| num < 0);
        let zeros1 = nums1[negatives1..].partition_point(|&num| num == 0);
        let positives1 = nums1.len() - negatives1 - zeros1;
        let negatives2 = nums2.partition_point(|&num| num < 0);
        let zeros2 = nums2[negatives2..].partition_point(|&num| num == 0);
        let positives2 = nums2.len() - negatives2 - zeros2;
        let negative_products = negatives1 * positives2 + positives1 * negatives2;
        let zero_products = zeros1 * nums2.len() + (negatives1 + positives1) * zeros2;
        let non_positive_products = negative_products + zero_products;
        if k > negative_products && k <= non_positive_products {
            return 0;
        }
        let non_positives1 = negatives1 + zeros1;
        let non_positives2 = negatives2 + zeros2;
        while product_size > 1 {
            let product_half = product_size / 2;
            product_size -= product_half;
            let product_middle = product_base + product_half;
            let mut not_greater_count = 0;
            match product_middle.cmp(&0) {
                Ordering::Equal => not_greater_count = non_positive_products,
                Ordering::Greater => {
                    if non_positive_products >= k {
                        continue;
                    }
                    not_greater_count = non_positive_products;
                    for &num1 in &nums1[..negatives1] {
                        // num1 is negative => pair is negative
                        let pair = (product_middle / num1 as i64) as i32;
                        not_greater_count +=
                            negatives2 - nums2[..negatives2].partition_point(|&num2| num2 < pair);
                    }
                    if not_greater_count >= k {
                        continue;
                    }
                    for &num1 in &nums1[non_positives1..] {
                        // num1 is positive => pair is positive
                        let pair = (product_middle / num1 as i64) as i32;
                        not_greater_count +=
                            nums2[non_positives2..].partition_point(|&num2| num2 <= pair);
                    }
                }
                Ordering::Less => {
                    for &num1 in &nums1[..negatives1] {
                        // num1 is negative => pair is positive
                        let num1 = num1 as i64;
                        let pair = ((product_middle + num1 + 1) / num1) as i32;
                        not_greater_count += positives2
                            - nums2[non_positives2..].partition_point(|&num2| num2 < pair);
                    }
                    if not_greater_count >= k {
                        continue;
                    }
                    for &num1 in &nums1[non_positives1..] {
                        // num1 is positive => pair is negative
                        let num1 = num1 as i64;
                        let pair = ((product_middle - num1 + 1) / num1) as i32;
                        not_greater_count +=
                            nums2[..negatives2].partition_point(|&num2| num2 <= pair);
                    }
                }
            }
            if not_greater_count < k {
                product_base = product_middle;
            }
        }
        product_base + 1
    }
}