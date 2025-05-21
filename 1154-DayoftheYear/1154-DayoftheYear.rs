// Last updated: 21.05.2025, 14:09:26
const WEEK_SIZE: usize = 7;
const WEEK_DAYS: [&str; WEEK_SIZE] = [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];
const MONTH_SIZE: usize = 12;
const DAYS_PRECEDING_MONTH: [i32; MONTH_SIZE] =
    [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
const LAST_DAY_OF_1970: i32 = 4;

impl Solution {
    pub fn day_of_the_week(day: i32, month: i32, year: i32) -> String {
        let years_delta = year - 1971;
        let mut week_day_shift = (LAST_DAY_OF_1970
            + years_delta
            + (years_delta + 2) / 4
            + DAYS_PRECEDING_MONTH[month as usize - 1]
            + day) as usize;
        if month > 2 && (year % 4 == 0 && year != 2100) {
            week_day_shift += 1;
        }
        WEEK_DAYS[week_day_shift % WEEK_SIZE].into()
    }
}