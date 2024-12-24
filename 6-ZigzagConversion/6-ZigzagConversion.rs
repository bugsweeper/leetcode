impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let s_bytes = s.as_bytes();
        let mut result = Vec::with_capacity(s_bytes.len());
        let num_rows = num_rows as usize;
        let zigzag_length = 2 * num_rows - 2;
        for i in (0..s_bytes.len()).step_by(zigzag_length) {
            result.push(s_bytes[i]);
        }
        for row_index in 1..num_rows - 1 {
            for zigzag_index in (0..s_bytes.len()).step_by(zigzag_length) {
                if zigzag_index + row_index < s_bytes.len() {
                    result.push(s_bytes[zigzag_index + row_index]);
                }
                let index = zigzag_index + zigzag_length - row_index;
                if index < s_bytes.len() {
                    result.push(s_bytes[index]);
                }
            }
        }
        for i in (num_rows - 1..s_bytes.len()).step_by(zigzag_length) {
            result.push(s_bytes[i]);
        }
        unsafe { String::from_utf8_unchecked(result) }
    }
}