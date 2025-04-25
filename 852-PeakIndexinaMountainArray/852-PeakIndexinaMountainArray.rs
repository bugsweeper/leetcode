// Last updated: 25.04.2025, 15:22:58
impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut base = 0;
        let mut size = arr.len();
        if arr[0] > arr[1] {
            return 1;
        }
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            if arr[middle] < arr[middle + 1] {
                base = middle;
            }
            size -= half;
        }
        base as i32 + 1
    }
}