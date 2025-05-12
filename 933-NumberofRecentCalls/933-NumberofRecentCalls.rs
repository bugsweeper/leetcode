// Last updated: 12.05.2025, 23:50:49
use std::cmp::Ordering;

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let mut prev = arr[0];
        let mut middle = 0;
        for (index, height) in arr.iter().skip(1).enumerate() {
            match prev.cmp(height) {
                Ordering::Greater => {
                    middle = index;
                    break;
                }
                Ordering::Equal => return false,
                _ => {}
            }
            prev = *height;
        }
        if middle == 0 || middle == arr.len() - 1 {
            return false;
        }
        for &height in &arr[middle + 1..] {
            if height >= prev {
                return false;
            }
            prev = height
        }
        true
    }
}