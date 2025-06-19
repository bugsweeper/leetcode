// Last updated: 19.06.2025, 15:17:06
impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        let bytes = coordinates.as_bytes();
        ((bytes[0] - b'a') & 1) == ((bytes[1] - b'0') & 1)
    }
}