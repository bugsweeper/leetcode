// Last updated: 16.06.2025, 14:41:56
impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let mut day_money = 6;
        let mut total_money = 0;
        for i in 0..n {
            if i % 7 == 0 {
                day_money -= 5;
            } else {
                day_money += 1;
            }
            total_money += day_money;
        }
        total_money
    }
}