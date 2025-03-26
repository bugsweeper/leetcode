// Last updated: 26.03.2025, 12:14:30
use std::cmp::Ordering;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut frequency = vec![0; 10_001];
        let mut min = i32::MAX;
        let mut max = 0;
        if x == 1 {
            for &cell in grid.iter().flatten() {
                frequency[cell as usize] += 1;
                min = min.min(cell);
                max = max.max(cell);
            }
        } else {
            let mut iter = grid.iter().flatten();
            let first = *iter.next().unwrap();
            min = first;
            max = first;
            frequency[first as usize] = 1;
            let correct_remainder = first % x;
            for &cell in iter {
                if cell % x != correct_remainder {
                    return -1;
                }
                frequency[cell as usize] += 1;
                min = min.min(cell);
                max = max.max(cell);
            }
        }
        let mut left = min as usize;
        let mut right = max as usize;
        let mut balance = 0;
        let ux = x as usize;
        let mut middle = left;
        while left <= right && left < frequency.len() && right < frequency.len() {
            match balance.cmp(&0) {
                Ordering::Equal => {
                    let left_frequency = frequency[left];
                    let right_frequency = frequency[right];
                    match left_frequency.cmp(&right_frequency) {
                        Ordering::Equal => {
                            if left_frequency > 0 {
                                middle = left;
                            }
                        }
                        Ordering::Greater => {
                            middle = left;
                        }
                        Ordering::Less => middle = right,
                    };
                    balance += left_frequency;
                    left += ux;
                    balance -= right_frequency;
                    right -= ux;
                }
                Ordering::Greater => {
                    let right_frequency = frequency[right];
                    if balance <= right_frequency {
                        middle = right;
                    }
                    balance -= right_frequency;
                    right -= ux;
                }
                Ordering::Less => {
                    let left_frequency = frequency[left];
                    if -balance <= left_frequency as i32 {
                        middle = left;
                    }
                    balance += left_frequency;
                    left += ux;
                }
            }
        }
        let middle = middle as i32;
        (min..=max)
            .step_by(x as usize)
            .map(|value| middle.abs_diff(value) as i32 / x * frequency[value as usize])
            .sum()
    }
}