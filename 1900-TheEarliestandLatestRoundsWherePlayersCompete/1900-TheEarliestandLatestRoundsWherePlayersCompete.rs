// Last updated: 12.07.2025, 16:14:45
use std::cmp::Ordering;
use std::collections::VecDeque;

/// Terminology description:
/// range is splitted into subranges by powerful players
/// for sequence 1 [2] 3 4 [5] 6 7 8  9  10 11 12 13
/// or sequence (1 [2] 3 4  5  6 7 8 [9] 10 11 12 13) they are 
/// `left` with size 1
/// `between` with size 2
/// `right` with size 8(4)
/// for round calculating each pair is marked depending on subrange winner could be placed
/// for our example they are
/// `border`: winner of pair 1-13 could become in `left` or `right` subrange
/// `center`: winners of pair 6-8 and 7 are guarantied in single `right`(`between`) subrange, so result is fixed
/// `middle`: winners of 3-11, 4-10 could become in `right` or `between` subrange
impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        let (mut left, between, mut right) = (first_player as usize - 1, (second_player - first_player - 1) as usize, (n - second_player) as usize);
        match left.cmp(&right) {
            Ordering::Equal => return vec![1, 1],
            Ordering::Greater => (left, right) = (right, left),
            _ => {},
        }
        let middle = if right <= left + between + 1 { (right - left).saturating_sub(1) } else { between };
        let mut queue = VecDeque::with_capacity(left * middle);
        queue.push_back((left, between, right, 1));
        let (mut earliest, mut latest) = (i32::MAX, i32::MAX);
        while let Some((mut prev_left, prev_between, mut prev_right, mut round)) = queue.pop_front() {
            if prev_left > prev_right {
                (prev_left, prev_right) = (prev_right, prev_left);
            }
            let border = prev_left;
            round += 1;
            match (prev_left + 1/* first_player itself */ + prev_between).cmp(&prev_right) {
                Ordering::Equal => {
                    // second_player exactly in the middle, hi is pair to himself and doesn't drop from round anyone
                    // so there is no `center` category of pairs
                    // as example sequence 1 [2] 3 4 5 6 [7] 8 9 10 11 12 13
                    let middle = prev_right - prev_left - 1;
                    for left in 0..=border {
                        for between in 0..=middle {
                            let right = border - left + middle - between;
                            if left == right {
                                latest = round;
                                if earliest == i32::MAX {
                                    earliest = round;
                                }
                            } else {
                                queue.push_back((left, between, right, round));
                            }
                        }
                    }
                },
                Ordering::Greater => {
                    let center = prev_left + prev_between - prev_right;
                    let middle = (prev_right - prev_left).saturating_sub(1);
                    for left in 0..=border {
                        for middle_in_right in 0..=middle {
                            let right = middle_in_right + border - left;
                            let between = middle - middle_in_right + center.div_ceil(2);
                            if left == right {
                                latest = round;
                                if earliest == i32::MAX {
                                    earliest = round;
                                }
                            } else {
                                queue.push_back((left, between, right, round));
                            }
                        }
                    }
                },
                Ordering::Less => {
                    let center = prev_right - prev_left - prev_between - 2;
                    let middle = prev_between;
                    for left in 0..=border {
                        for between in 0..=middle {
                            let right = border - left + middle - between + center.div_ceil(2);
                            if left == right {
                                latest = round;
                                if earliest == i32::MAX {
                                    earliest = round;
                                }
                            } else {
                                queue.push_back((left, between, right, round));
                            }
                        }
                    }
                }
            }
        }
        vec![earliest, latest]
    }
}