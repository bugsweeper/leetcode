// Last updated: 05.07.2025, 12:54:46
impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut freq = vec![0; arr.len() + 1];
        for num in arr {
            if let Some(freq) = freq.get_mut(num as usize) {
                *freq += 1;
            }
        }
        for (index, freq) in freq.into_iter().enumerate().skip(1).rev() {
            if index as i32 == freq {
                return freq;
            }
        }
        -1
    }
}