// Last updated: 10.04.2025, 13:17:19
use std::cmp::Ordering;

fn powerful_count(mut number: i64, limit: i64) -> i64 {
    if limit == 9 {
        return number;
    }
    if number == 0 {
        return 1;
    }
    let mut multiplier = 1;
    let mut count = 1;
    while number > 0 {
        let mut digit = number % 10;
        if digit > limit {
            count = multiplier;
            digit = limit;
        }
        count += digit * multiplier;
        multiplier *= limit + 1;
        number /= 10;
    }
    count
}

impl Solution {
    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let suffix: i64 = s.parse().unwrap();
        let step = 10_i64.pow(s.len() as u32);
        let limit = limit as i64;
        let mut first = start - start % step + suffix;
        if first < start {
            first += step;
        }
        first /= step;
        let mut last = finish - finish % step + suffix;
        if last > finish {
            last -= step;
            if last < 0 {
                return 0;
            }
        }
        if limit == 9 {
            last /= step;
        } else {
            // Ensure that `last` contains only valid digits
            let mut unchecked_last = last / step;
            let mut multiplier = 1;
            last = 0;
            while unchecked_last > 0 {
                let mut digit = unchecked_last % 10;
                if digit > limit {
                    digit = limit;
                    last = (multiplier - 1) / 9 * limit;
                }
                last += digit * multiplier;
                multiplier *= 10;
                unchecked_last /= 10;
            }
        }
        let mut first_contains_forbidden_digits = false;
        let mut first_for_check = first;
        while first_for_check > 0 {
            if first_for_check % 10 > limit {
                first_contains_forbidden_digits = true;
                break;
            }
            first_for_check /= 10;
        }
        match last.cmp(&first) {
            Ordering::Less => 0,
            _ => {
                powerful_count(last, limit) - powerful_count(first, limit)
                    + if first_contains_forbidden_digits {
                        0
                    } else {
                        1
                    }
            }
        }
    }
}