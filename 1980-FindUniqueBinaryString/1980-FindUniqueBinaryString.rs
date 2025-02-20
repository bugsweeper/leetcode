fn binary_string2num(binary_string: &str) -> usize {
    let mut result = 0;
    for &digit in binary_string.as_bytes() {
        result <<= 1;
        if digit == b'1' {
            result += 1;
        }
    }
    result
}

fn num2binary_string(num: usize, length: usize) -> String {
    let mut result = Vec::with_capacity(length);
    let mut mask = 1usize << (length - 1);
    for _ in 0..length {
        result.push(if num & mask == 0 { b'0' } else { b'1' });
        mask >>= 1;
    }
    unsafe { String::from_utf8_unchecked(result) }
}

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let set = nums
            .iter()
            .map(|binary_string| binary_string2num(binary_string.as_str()))
            .collect::<std::collections::HashSet<_>>();
        for num in 0..nums.len() + 1 {
            if !set.contains(&num) {
                return num2binary_string(num, nums.len());
            }
        }
        String::new()
    }
}