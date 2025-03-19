impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let bytes = s.as_bytes();
        let real_length = bytes.iter().filter(|byte| **byte != b'-').count();
        if real_length == 0 {
            return String::new();
        }
        let mut iter = bytes.iter().filter_map(|byte| if *byte != b'-' {Some(byte.to_ascii_uppercase())} else {None});
        let mut result = String::new();
        for _ in 0..(real_length - 1) % k as usize + 1 {
            result.push(iter.next().unwrap() as char);
        }
        while let Some(first_group_item) = iter.next() {
            result.push('-');
            result.push(first_group_item as char);
            for _ in 1..k {
                result.push(iter.next().unwrap() as char);
            }
        }
        result
    }
}