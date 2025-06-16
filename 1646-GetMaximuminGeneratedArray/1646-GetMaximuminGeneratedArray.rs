// Last updated: 16.06.2025, 15:07:08
impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut prev_num = first;
        let mut decoded = Vec::with_capacity(encoded.len() + 1);
        decoded.push(prev_num);
        for num in encoded {
            prev_num ^= num;
            decoded.push(prev_num);
        }
        decoded
    }
}