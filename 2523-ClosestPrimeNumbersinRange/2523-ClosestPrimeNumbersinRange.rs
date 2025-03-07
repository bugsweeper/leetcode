impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let right = right as usize;
        let mut primes = vec![true; right + 1];
        primes[1] = false;
        for i in 2..=right {
            if i * i > right {
                break;
            }
            if primes[i] {
                for num in primes[i * i..=right].iter_mut().step_by(i) {
                    *num = false
                }
            }
        }
        let mut iter = primes
            .into_iter()
            .enumerate()
            .skip(left as usize)
            .filter_map(|(index, is_prime)| if is_prime { Some(index as i32) } else { None });
        if let (Some(mut num1), Some(mut num2)) = (iter.next(), iter.next()) {
            let mut prev_num = num2;
            let mut prev_diff = num2 - num1;
            for next_num in iter {
                let next_diff = next_num - prev_num;
                if next_diff < prev_diff {
                    (num1, num2, prev_diff) = (prev_num, next_num, next_diff)
                }
                prev_num = next_num;
            }
            vec![num1, num2]
        } else {
            vec![-1, -1]
        }
    }
}