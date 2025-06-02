// Last updated: 02.06.2025, 10:39:45
use std::cmp::Ordering;

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        if ratings.len() == 1 {
            return 1;
        }
        if ratings.len() == 2 {
            return if ratings[0] == ratings[1] { 2 } else { 3 };
        }
        let mut candies = vec![1; ratings.len()];
        let mut current = 0;
        while current < ratings.len() - 1 {
            match ratings[current].cmp(&ratings[current + 1]) {
                Ordering::Equal => current += 1,
                Ordering::Greater => {
                    let mut end = ratings.len() - 1;
                    // searching end of the increasing sequence
                    for i in current + 1..ratings.len() - 1 {
                        if ratings[i] <= ratings[i + 1] {
                            end = i;
                            break;
                        }
                    }
                    for (i, candy) in candies.iter_mut().enumerate().take(end + 1).skip(current) {
                        *candy = *candy.max(&mut (end - i + 1));
                    }
                    current = end;
                }
                Ordering::Less => {
                    let mut end = ratings.len() - 1;
                    // searching end of the decreasing sequence
                    for i in current + 1..ratings.len() - 1 {
                        if ratings[i] >= ratings[i + 1] {
                            end = i;
                            break;
                        }
                    }
                    for (i, candy) in candies
                        .iter_mut()
                        .enumerate()
                        .take(end + 1)
                        .skip(current + 1)
                    {
                        *candy = i - current + 1;
                    }
                    current = end;
                }
            }
        }
        candies.iter().sum::<usize>() as i32
    }
}
