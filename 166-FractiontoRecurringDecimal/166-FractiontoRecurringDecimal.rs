// Last updated: 24.09.2025, 15:56:32
use std::collections::HashMap;

const MAX_LEN: usize = 10_000;

impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".into();
        }
        let (numerator, denominator) = (numerator as i64, denominator as i64);
        let add_minus = denominator < 0 && numerator > 0;
        let (quotient, mut remainder) = (
            numerator / denominator,
            (numerator % denominator).unsigned_abs() as usize,
        );
        let quotient_str = quotient.to_string();
        if remainder == 0 {
            return quotient_str;
        }
        let denominator = denominator.unsigned_abs() as usize;
        remainder *= 10;
        let mut reminder_position = HashMap::with_capacity(MAX_LEN);
        let mut fractional = Vec::with_capacity(MAX_LEN);
        for i in 0..MAX_LEN {
            if remainder == 0 {
                break;
            }
            if let Some(prev_reminder_position) = reminder_position.insert(remainder, i) {
                fractional.insert(prev_reminder_position, b'(');
                fractional.push(b')');
                break;
            }
            if remainder < denominator {
                remainder *= 10;
                fractional.push(b'0');
                continue;
            }
            fractional.push((remainder / denominator) as u8 + b'0');
            remainder = remainder % denominator * 10;
        }
        let mut answer = String::with_capacity(
            if add_minus { 2 } else { 1 } + quotient_str.len() + fractional.len(),
        );
        if add_minus {
            answer.push('-');
        }
        answer.push_str(&quotient_str);
        answer.push('.');
        answer.extend(fractional.into_iter().map(|byte| byte as char));
        answer
    }
}