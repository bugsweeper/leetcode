// Last updated: 01.08.2025, 12:04:36
impl Solution {
    pub fn cells_in_range(s: String) -> Vec<String> {
        let s = s.as_bytes();
        let (cs, ce, rs, re) = (s[0], s[3], s[1], s[4]);
        let mut answer = Vec::with_capacity((ce - cs + 1) as usize * (re - rs + 1) as usize);
        for col in s[0]..=s[3] {
            for row in s[1]..=s[4] {
                let mut cell = String::with_capacity(2);
                cell.push(col as char);
                cell.push(row as char);
                answer.push(cell);
            }
        }
        answer
    }
}