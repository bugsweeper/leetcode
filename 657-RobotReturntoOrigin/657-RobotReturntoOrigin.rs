// Last updated: 14.04.2025, 13:43:50
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        if moves.len() & 1 == 1 {
            return false;
        }
        let bytes = moves.as_bytes();
        let (mut x, mut y) = (0, 0);
        for &byte in bytes {
            match byte {
                b'R' => x += 1,
                b'L' => x -= 1,
                b'U' => y += 1,
                b'D' => y -= 1,
                _ => unimplemented!(),
            }
        }
        (x, y) == (0, 0)
    }
}