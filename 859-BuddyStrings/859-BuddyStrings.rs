// Last updated: 06.05.2025, 13:36:42
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

fn get_statistic(bytes: &[u8]) -> [usize; ABC_LEN] {
    let mut statistic = [0; ABC_LEN];
    for &byte in bytes {
        statistic[(byte - b'a') as usize] += 1;
    }
    statistic
}

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() || s.len() < 2 {
            return false;
        }
        let statistic = get_statistic(s.as_bytes());
        if statistic != get_statistic(goal.as_bytes()) {
            return false;
        }
        let mut iter = s
            .as_bytes()
            .iter()
            .zip(goal.as_bytes())
            .filter_map(|(s, t)| if s == t { None } else { Some((s, t)) });
        match (iter.next(), iter.next(), iter.next()) {
            (None, _, _) => statistic.into_iter().any(|count| count > 1),
            (Some((s1, t1)), Some((s2, t2)), None) => s1 == t2 && s2 == t1,
            _ => false,
        }
    }
}