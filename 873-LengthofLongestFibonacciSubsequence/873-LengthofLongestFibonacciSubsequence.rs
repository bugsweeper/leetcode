impl Solution {
    pub fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
        let mut max_subseq_len = 0;
        for i in 0..arr.len() - 2 {
            if arr.len() - i <= max_subseq_len {
                break;
            }
            for j in i + 1..arr.len() - 1.max(max_subseq_len.saturating_sub(2)) {
                let mut a = arr[i];
                let mut b = arr[j];
                let mut c = a + b;
                if let Ok(mut c_index) = arr[j + 1..].binary_search(&c) {
                    c_index += j + 1;
                    let mut subseq_len = 3;
                    (a, b, c) = (b, c, b + c);
                    while let Ok(next_c_index) = arr[c_index + 1..].binary_search(&c) {
                        c_index += next_c_index + 1;
                        subseq_len += 1;
                        (a, b, c) = (b, c, b + c);
                    }
                    max_subseq_len = max_subseq_len.max(subseq_len);
                }
            }
        }
        max_subseq_len as i32
    }
}