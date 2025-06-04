// Last updated: 04.06.2025, 12:59:14
impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let m = m as usize;
        let k = k as usize;
        for shift in 0..m {
            if arr.len() < shift + m * k {
                break;
            }
            let mut iter = arr[shift..].chunks_exact(m);
            let mut prev_slice = iter.next().unwrap();
            let mut count = 1;
            for slice in iter {
                if slice == prev_slice {
                    count += 1;
                    if count == k {
                        return true;
                    }
                } else {
                    prev_slice = slice;
                    count = 1;
                }
            }
        }
        false
    }
}