// Last updated: 04.06.2025, 12:20:42
impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let first = *rounds.first().unwrap();
        let last = *rounds.last().unwrap();
        if last >= first {
            (first..=last).collect::<Vec<_>>()
        } else {
            (1..=last).chain(first..=n).collect::<Vec<_>>()
        }
    }
}