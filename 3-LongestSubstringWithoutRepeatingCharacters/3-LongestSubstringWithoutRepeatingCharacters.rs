impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut left = 0;
        let mut right = 1;
        let mut answer = 1;
        let s_bytes = s.as_bytes();
        let mut seen = vec![false; 256];
        // Safety: removed redundant checks if problem constraints are valid
        *(unsafe {seen.get_unchecked_mut(*s_bytes.get_unchecked(0) as usize)}) = true;
        loop {
            if right >= s_bytes.len() {
                break;
            }
            if *(unsafe {seen.get_unchecked(*s_bytes.get_unchecked(right) as usize)}) {
                *(unsafe {seen.get_unchecked_mut(*s_bytes.get_unchecked(left) as usize)}) = false;
                left += 1;
            } else {
                *(unsafe {seen.get_unchecked_mut(*s_bytes.get_unchecked(right) as usize)}) = true;
                right += 1;
                answer = answer.max(right - left);
            }
        }
        answer as i32
    }
}