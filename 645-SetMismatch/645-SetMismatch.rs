// Last updated: 14.04.2025, 11:59:29
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = vec![false; nums.len()];
        let mut duplicated = 0;
        for num in nums {
            let seen = seen.get_mut(num as usize - 1).unwrap();
            if *seen {
                duplicated = num;
            } else {
                *seen = true;
            }
        }
        vec![
            duplicated,
            seen.into_iter()
                .enumerate()
                .filter_map(|(index, seen)| if seen { None } else { Some(index) })
                .next()
                .unwrap() as i32 + 1,
        ]
    }
}
