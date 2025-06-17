// Last updated: 17.06.2025, 13:55:18
use std::cell::RefCell;

const MODULO: i64 = 1_000_000_007;
thread_local! {
    // Cache in case of running multiple tests in same thread sequentaly
    pub static FACTORIALS: RefCell<Vec<i64>> = {
        let mut factorials = Vec::with_capacity(100_000);
        factorials.push(1); // 0!
        factorials.push(1); // 1!
        RefCell::new(factorials)
    };
}

#[inline(always)]
fn pow(mut base: i64, mut power: i32) -> i64 {
    let mut result = 1;
    while power > 0 {
        if power & 1 == 1 {
            result = (result * base) % MODULO;
        }
        base = (base * base) % MODULO;
        power >>= 1;
    }
    result
}

fn factorial(n: i32) -> i64 {
    FACTORIALS.with_borrow_mut(|factorials| {
        let n = n as usize;
        if n < factorials.len() {
            return factorials[n];
        }
        let mut factorial = *factorials.last().unwrap();
        for multiplier in factorials.len() as i64..=n as i64 {
            factorial = (factorial * multiplier) % MODULO;
            factorials.push(factorial);
        }
        factorial
    })
}

#[inline(always)]
fn inverse(factorial: i64) -> i64 {
    pow(factorial, MODULO as i32 - 2)
}

#[inline(always)]
fn c(k: i32, n: i32) -> i64 {
    if k == 0 || k == n {
        return 1;
    }
    (factorial(n) * inverse(factorial(k))) % MODULO * inverse(factorial(n - k)) % MODULO
}

impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        if m == 1 && k < n - 1 {
            return 0;
        }
        let mut combinations = m as i64 * c(k, n - 1) % MODULO;
        if m > 2 {
            combinations = combinations * pow(m as i64 - 1, n - k - 1) % MODULO;
        }
        combinations as i32
    }
}