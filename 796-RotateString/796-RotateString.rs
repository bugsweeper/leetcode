// Last updated: 28.04.2025, 14:58:44
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        for i in 0..s.len() {
            if s[..i] == goal[goal.len() - i..] && s[i..] == goal[..goal.len() - i] {
                return true;
            }
        }
        false
    }
}