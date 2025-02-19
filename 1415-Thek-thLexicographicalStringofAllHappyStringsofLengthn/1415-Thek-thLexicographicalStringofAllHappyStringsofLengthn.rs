const RELATIVE_SHIFT: [[usize;2];3] = [
    [1, 2],
    [0, 2],
    [0, 1]
];

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        if k > (1 << (n - 1)) * 3 {
            return String::new();
        }
        let mut result = Vec::with_capacity(n as usize);
        let mut combinations = 1_usize << (n - 1);
        let mut previous_shift = (k as usize - 1) / combinations;
        let mut k = (k as usize - 1) % combinations;
        result.push(previous_shift as u8 + b'a');
        for i in 0..n - 1 {
            combinations >>= 1;
            let shift = RELATIVE_SHIFT[previous_shift][k / combinations];
            result.push(shift as u8 + b'a');
            k %= combinations;
            previous_shift = shift;
        }
        unsafe{String::from_utf8_unchecked(result)}
    }
}