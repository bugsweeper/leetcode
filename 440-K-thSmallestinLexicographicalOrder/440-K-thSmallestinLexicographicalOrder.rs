// Last updated: 09.06.2025, 16:39:05
fn find_kth_full_subsequence_number(number_start: i32, mut k: i32, step: i32) -> i32 {
    if k == 0 {
        return number_start;
    }
    k -= 1;
    let digit = k / step;
    find_kth_full_subsequence_number(number_start * 10 + digit, k - digit * step, step / 10)
}

fn find_kth_partial_subsequence_number(
    number_start: i32,
    start_digits_from: i32,
    mut k: i32,
    n: i32,
    mut combinations: i32,
    power_of_10: i32,
    bottom_step: i32,
) -> i32 {
    if k == 0 {
        return number_start;
    }
    k -= 1;
    combinations -= 1;
    let first_digit = n / power_of_10;
    let mut skip_bottom = 0;
    let top_step = bottom_step - power_of_10;
    for digit in start_digits_from..first_digit {
        let next_skip = skip_bottom + bottom_step;
        if next_skip <= k {
            skip_bottom = next_skip;
        } else {
            return find_kth_full_subsequence_number(
                number_start * 10 + digit,
                k - skip_bottom,
                top_step,
            );
        }
    }
    let mut skip_top = combinations;
    for digit in (first_digit + 1..=9).rev() {
        let next_skip = skip_top - top_step;
        if next_skip < k {
            return find_kth_full_subsequence_number(
                number_start * 10 + digit,
                k - next_skip - 1,
                top_step / 10,
            );
        } else {
            skip_top = next_skip;
        }
    }
    find_kth_partial_subsequence_number(
        number_start * 10 + first_digit,
        0,
        k - skip_bottom,
        n % power_of_10,
        skip_top - skip_bottom,
        power_of_10 / 10,
        top_step,
    )
}

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let power_of_n = n.ilog10();
        let power_of_10 = 10_i32.pow(power_of_n);
        let top_subsequence_size = (power_of_10 - 1) / 9;
        let bottom_subsequence_size = power_of_10 + top_subsequence_size;
        find_kth_partial_subsequence_number(0, 1, k, n, n, power_of_10, bottom_subsequence_size)
    }
}