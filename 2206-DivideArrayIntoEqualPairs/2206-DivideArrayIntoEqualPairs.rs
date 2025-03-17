impl Solution {
    pub fn divide_array(nums: Vec<i32>) -> bool {
        let mut num_count = [0; 501];
        for num in nums {
            num_count[num as usize] += 1;
        }
        num_count.into_iter().all(|num_count| num_count & 1 == 0)
    }
}