// Last updated: 03.06.2025, 14:03:33
const MONTH: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

impl Solution {
    pub fn reformat_date(date: String) -> String {
        let day_len = date.len() - 11;
        format!("{}-{:02}-{:0>2}", &date[day_len + 7..], MONTH.into_iter().position(|month| month == &date[day_len + 3..day_len + 6]).unwrap() + 1, &date[..day_len])
    }
}