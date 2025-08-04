// Last updated: 04.08.2025, 13:03:43
#[inline]
fn to_minutes(time: String) -> i32 {
    time[..2].parse::<i32>().unwrap() * 60 + time[3..].parse::<i32>().unwrap()
}

impl Solution {
    pub fn convert_time(current: String, correct: String) -> i32 {
        let mut remaining = to_minutes(correct) - to_minutes(current);
        let mut operations = remaining / 60;
        remaining %= 60;
        operations += remaining / 15;
        remaining %= 15;
        operations += remaining / 5;
        remaining %= 5;
        operations + remaining
    }
}