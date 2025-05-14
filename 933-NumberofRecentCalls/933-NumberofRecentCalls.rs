// Last updated: 14.05.2025, 22:29:30
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        if *nums.first().unwrap() >= 0 {
            return nums.into_iter().map(|num| num * num).collect();
        }
        if *nums.last().unwrap() < 0 {
            return nums.into_iter().rev().map(|num| num * num).collect();
        }
        let first_nonnegative_index = nums.partition_point(|&num| num < 0);
        let mut nonnegative_iter = nums.iter().skip(first_nonnegative_index);
        let mut negative_iter = nums.iter().rev().skip(nums.len() - first_nonnegative_index);
        let mut sorted_squares = Vec::with_capacity(nums.len());
        let mut nonnegative = *nonnegative_iter.next().unwrap();
        let mut negative = *negative_iter.next().unwrap();
        loop {
            if nonnegative > -negative {
                sorted_squares.push(negative * negative);
                if let Some(&num) = negative_iter.next() {
                    negative = num;
                } else {
                    sorted_squares.push(nonnegative * nonnegative);
                    sorted_squares.extend(nonnegative_iter.map(|&num| num * num));
                    break;
                }
            } else {
                sorted_squares.push(nonnegative * nonnegative);
                if let Some(&num) = nonnegative_iter.next() {
                    nonnegative = num;
                } else {
                    sorted_squares.push(negative * negative);
                    sorted_squares.extend(negative_iter.map(|&num| num * num));
                    break;
                }
            }
        }
        sorted_squares
    }
}