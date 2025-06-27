// Last updated: 27.06.2025, 11:31:08
fn add_sub_permutation(index: usize, mut nums: Vec<i32>, result: &mut Vec<Vec<i32>>) {
    let next_index = index + 1;
    if next_index == nums.len() - 1 {
        result.push(nums.clone());
        nums.swap(index, next_index);
        result.push(nums);
    } else {
        let last = nums.len() - 1;
        add_sub_permutation(next_index, nums.clone(), result);
        for i in next_index..last {
            let mut nums_clone = nums.clone();
            nums_clone.swap(index, i);
            add_sub_permutation(next_index, nums_clone, result);
        }
        nums.swap(index, last);
        add_sub_permutation(next_index, nums, result);
    }
}

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() == 1 {
            return vec![nums];
        }
        let mut result = Vec::with_capacity((1..nums.len()).product());
        add_sub_permutation(0, nums, &mut result);
        result
    }
}