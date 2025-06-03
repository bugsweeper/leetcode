// Last updated: 03.06.2025, 14:55:53
impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut base = 0;
        let mut size = arr.len();
        while size > 1 {
            let half = size >> 1;
            let middle = base + half;
            if arr[middle] - k <= middle as i32 {
                base = middle;
            }
            size -= half;
        }
        k + base as i32 + if arr[base] - k <= base as i32 { 1 } else { 0 }
    }
}