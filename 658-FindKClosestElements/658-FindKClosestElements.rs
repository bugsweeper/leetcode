// Last updated: 02.05.2025, 13:35:26
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        if k == arr.len() {
            return arr;
        }
        let mut middle = arr.partition_point(|&num| num <= x);
        if middle == 0 {
            return arr[..k].into();
        }
        if middle == arr.len() {
            return arr[arr.len() - k..].into();
        }
        if middle > 0 && x - arr[middle - 1] <= arr[middle] - x {
            middle -= 1;
        }
        let mut left = middle;
        let mut right = middle + 1;
        while right - left < k {
            if left == 0 {
                return arr[..k].into();
            }
            if right >= arr.len() {
                return arr[arr.len() - k..].into();
            }
            if (arr[left - 1] - x).abs() > (arr[right] - x).abs() {
                right += 1;
            } else {
                left -= 1;
            }
        }
        arr[left..right].into()
    }
}