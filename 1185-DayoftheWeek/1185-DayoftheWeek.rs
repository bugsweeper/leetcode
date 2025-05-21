// Last updated: 21.05.2025, 14:39:34
const ABC_LEN: usize = (b'z' - b'a' + 1) as usize;

fn count_chars(text: &[u8]) -> [i32; ABC_LEN] {
    let mut chars_count = [0; ABC_LEN];
    for &c in text {
        chars_count[(c - b'a') as usize] += 1;
    }
    chars_count
}

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let goal_count = count_chars("balloon".as_bytes());
        let chars_count = count_chars(text.as_bytes());
        goal_count.iter().zip(chars_count.iter()).filter_map(|(goal, current)| {
            if *goal > 0 {
                Some(current / goal)
            } else {
                None
            }
        }).min().unwrap() as i32
    }
}