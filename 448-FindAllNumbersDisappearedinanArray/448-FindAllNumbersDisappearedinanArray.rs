impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut seen = vec![false; nums.len() + 1];
        for num in nums {
            seen[num as usize] = true;
        }
        seen.into_iter()
            .enumerate()
            .skip(1)
            .filter_map(|(index, seen)| if seen { None } else { Some(index as i32) })
            .collect()
    }
}