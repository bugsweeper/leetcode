// Last updated: 10.06.2025, 11:37:56
impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {
        let mut freq = [0; 1001];
        let mut remains = nums.len();
        for num in nums {
            freq[num as usize] += 1;
        }
        for (num, freq) in freq.into_iter().enumerate() {
            if num == remains {
                return num as i32
            }
            remains -= freq;
        }
        -1
    }
}