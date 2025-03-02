use std::cmp::Ordering;

impl Solution {
    pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(nums1.len() + nums2.len());
        let mut iter1 = nums1.into_iter();
        let mut iter2 = nums2.into_iter();
        let mut num1 = iter1.next().unwrap();
        let mut num2 = iter2.next().unwrap();
        loop {
            match num1[0].cmp(&num2[0]) {
                Ordering::Less => {
                    result.push(num1);
                    if let Some(num) = iter1.next() {
                        num1 = num;
                    } else {
                        result.push(num2);
                        result.extend(iter2);
                        break;
                    }
                },
                Ordering::Equal => {
                    num1[1] += num2[1];
                    result.push(num1);
                    if let Some(num) = iter1.next() {
                        num1 = num;
                    } else {
                        result.extend(iter2);
                        break;
                    }
                    if let Some(num) = iter2.next() {
                        num2 = num;
                    } else {
                        result.push(num1);
                        result.extend(iter1);
                        break;
                    }
                },
                Ordering::Greater => {
                    result.push(num2);
                    if let Some(num) = iter2.next() {
                        num2 = num;
                    } else {
                        result.push(num1);
                        result.extend(iter1);
                        break;
                    }
                },
            }
        }
        result
    }
}