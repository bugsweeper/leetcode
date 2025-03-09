impl Solution {
    pub fn number_of_alternating_groups(colors: Vec<i32>, k: i32) -> i32 {
        let mut result = 0;
        let mut start = 0;
        let n = colors.len();
        let k = k as usize;
        let last = n + k - 1;
        let mut end = start + 1;
        let mut prev_color = colors[start];
        while end <= last {
            let color = colors[end % n];
            if prev_color == color {
                let len = end - start;
                if len >= k {
                    result += len - k + 1
                }
                start = end;
                if last - start < k {
                    return result as i32;
                }
            }
            prev_color = color;
            end += 1;
        }
        let len = end - start;
        if len > k {
            result += len - k
        }
        result as i32
    }
}