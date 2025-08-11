// Last updated: 11.08.2025, 13:51:06
impl Solution {
    pub fn calculate_tax(brackets: Vec<Vec<i32>>, income: i32) -> f64 {
        let mut lower = 0;
        let mut tax = 0.;
        for bracket in brackets {
            let (upper, percent) = (bracket[0], bracket[1]);
            if upper > income {
                tax += ((income - lower) * percent) as f64 / 100.;
                break;
            } else {
                tax += ((upper - lower) * percent) as f64 / 100.;
            }
            lower = upper;
        }
        tax
    }
}