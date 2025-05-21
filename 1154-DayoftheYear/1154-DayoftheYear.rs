// Last updated: 21.05.2025, 11:20:11
// const array with days preceding before each month
const MONTHS_PRECEDING_DAYS: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

impl Solution {
    pub fn day_of_year(date: String) -> i32 {
        let year = date[..4].parse::<i32>().unwrap();
        let month = date[5..7].parse::<usize>().unwrap() - 1;
        let day = date[8..].parse::<i32>().unwrap();
        let mut day_of_year = day + MONTHS_PRECEDING_DAYS[month];
        if month > 1 && (year % 400 == 0 || year % 4 == 0 && year % 100 != 0)  {
            day_of_year += 1;
        }
        day_of_year
    }
}