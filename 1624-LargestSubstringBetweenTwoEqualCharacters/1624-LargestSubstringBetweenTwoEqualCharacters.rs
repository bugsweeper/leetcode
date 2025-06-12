// Last updated: 12.06.2025, 16:33:07
impl Solution {
    pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
        let mut freq = [usize::MAX; 101];
        for (index, num) in arr.into_iter().enumerate() {
            freq[num as usize] = index;
        }
        for piece in pieces {
            let first_index = freq[piece[0] as usize];
            if first_index == usize::MAX {
                return false;
            }
            for (index, num) in piece.into_iter().enumerate().skip(1) {
                if first_index + index != freq[num as usize] {
                    return false;
                }
            }
        }
        true
    }
}