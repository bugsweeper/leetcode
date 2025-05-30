// Last updated: 30.05.2025, 16:07:51
const PRECEDING_DAYS: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

#[inline(always)]
fn to_days(date: String) -> i32 {
    let (mut year, month, day) = (date[..4].parse::<i32>().unwrap(), date[5..7].parse::<usize>().unwrap() - 1, date[8..].parse::<i32>().unwrap());
    let mut days = year * 365 + (year - 1) / 4 + PRECEDING_DAYS[month] + day;
    if year % 4 == 0 && month > 1 && year != 2100 {
        days += 1;
    }
    days
}

impl Solution {
    pub fn days_between_dates(date1: String, date2: String) -> i32 {
        (to_days(date1) - to_days(date2)).abs()
    }
}