// Last updated: 09.05.2025, 21:30:28
use std::collections::HashMap;

const MODULO: usize = 1_000_000_007;
const FACTORIAL: [usize; 81] = [
    1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800, 39916800, 479001600, 227020758,
    178290591, 674358851, 789741546, 425606191, 660911389, 557316307, 146326063, 72847302,
    602640637, 860734560, 657629300, 440732388, 459042011, 394134213, 35757887, 36978716,
    109361473, 390205642, 486580460, 57155068, 943272305, 14530444, 523095984, 354551275,
    472948359, 444985875, 799434881, 776829897, 626855450, 954784168, 10503098, 472639410,
    741412713, 846397273, 627068824, 726372166, 318608048, 249010336, 948537388, 272481214,
    713985458, 269199917, 75195247, 286129051, 595484846, 133605669, 16340084, 996745124,
    798197261, 286427093, 331333826, 536698543, 422103593, 280940535, 103956247, 172980994,
    108669496, 715534167, 518459667, 847555432, 719101534, 932614679, 878715114, 661063309,
    562937745, 472081547, 766523501,
];
const INVERTED_FACTORIAL: [usize; 81] = [
    1, 1, 500000004, 166666668, 41666667, 808333339, 301388891, 900198419, 487524805, 831947206,
    283194722, 571199524, 380933296, 490841026, 320774361, 821384963, 738836565, 514049213,
    639669405, 402087866, 120104394, 862862120, 130130097, 179570875, 799148792, 791965957,
    761229465, 917082579, 282752951, 699405279, 123313510, 649139150, 957785605, 604781386,
    958964165, 170256120, 671396008, 261389083, 243720767, 211377457, 230284438, 469031331,
    654024560, 317535457, 393580354, 297635121, 202122504, 366002609, 861791727, 854322286,
    57086446, 138374245, 98814890, 662241795, 956708188, 762849245, 960050886, 16842998, 379600744,
    752196628, 279203279, 250478744, 971781922, 888440989, 92006891, 262953954, 367620517,
    856233148, 659650492, 328400734, 261834298, 102279357, 626420551, 159266036, 785936033,
    757145819, 470488764, 58058296, 590487931, 361904913, 517023815,
];

#[inline]
fn c(m: usize, n: usize) -> usize {
    (((FACTORIAL[n] * INVERTED_FACTORIAL[n - m]) % MODULO) * INVERTED_FACTORIAL[m]) % MODULO
}

#[derive(Clone, Copy, Debug, Default)]
struct HalfState {
    digit_count: usize,
    collected_sum: usize,
}

impl HalfState {
    #[inline]
    fn next(&self, current_digit: usize, digit_count: usize) -> Self {
        Self {
            digit_count: self.digit_count + digit_count,
            collected_sum: self.collected_sum + digit_count * current_digit,
        }
    }
    #[inline]
    fn overflow(&self, target_digit_count: usize, fixed_state: &FixedState) -> bool {
        self.digit_count > target_digit_count || self.collected_sum > fixed_state.target_sum
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct State {
    even: HalfState,
    odd: HalfState,
}

impl State {
    #[inline]
    fn next(
        &self,
        current_digit: usize,
        even_digit_count: usize,
        fixed_state: &FixedState,
    ) -> Self {
        Self {
            even: self.even.next(current_digit, even_digit_count),
            odd: self.odd.next(
                current_digit,
                fixed_state.digit_count[current_digit] - even_digit_count,
            ),
        }
    }
}

struct FixedState {
    digit_count: [usize; 10],
    target_even_count: usize,
    target_odd_count: usize,
    target_sum: usize,
}

fn count_recursive(
    processed_digits: usize,
    state: State,
    fixed_state: &FixedState,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(count) = cache.get(&(
        processed_digits,
        state.even.collected_sum,
        state.even.digit_count,
    )) {
        return *count;
    }
    let mut count = 0;
    for even_digit_count in 0..=fixed_state.digit_count[processed_digits] {
        let next_state = state.next(processed_digits, even_digit_count, fixed_state);
        if next_state
            .even
            .overflow(fixed_state.target_even_count, fixed_state)
        {
            break;
        }
        if next_state
            .odd
            .overflow(fixed_state.target_odd_count, fixed_state)
        {
            continue;
        }
        count += if processed_digits == 9 {
            1
        } else {
            (c(
                even_digit_count,
                fixed_state.target_even_count - state.even.digit_count,
            ) * c(
                fixed_state.digit_count[processed_digits] - even_digit_count,
                fixed_state.target_odd_count - state.odd.digit_count,
            )) % MODULO
                * count_recursive(processed_digits + 1, next_state, fixed_state, cache)
        };
    }
    count %= MODULO;
    cache.insert(
        (
            processed_digits,
            state.even.collected_sum,
            state.even.digit_count,
        ),
        count,
    );
    count
}

impl Solution {
    pub fn count_balanced_permutations(num: String) -> i32 {
        let mut digit_count = [0; 10];
        let mut sum = 0;
        for &digit in num.as_bytes() {
            let digit = (digit - b'0') as usize;
            digit_count[digit] += 1;
            sum += digit;
        }
        if sum % 2 == 1 {
            return 0;
        }
        let target_sum = sum / 2;
        let target_even_count = num.len().div_ceil(2);
        let target_odd_count = num.len() / 2;
        // cache for dynamic programming contains (current_digit, sum remaining, count remaining) as a key and number of permutation as value
        let mut cache = HashMap::with_capacity(5000);
        let fixed_state = FixedState {
            digit_count,
            target_even_count,
            target_odd_count,
            target_sum,
        };
        count_recursive(0, State::default(), &fixed_state, &mut cache) as i32
    }
}