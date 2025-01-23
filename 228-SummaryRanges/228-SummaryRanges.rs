impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return Vec::new();
        }
        let mut start = *unsafe { nums.get_unchecked(0) };
        let mut previous = start;
        let mut result = vec![];
        for num in nums.into_iter().skip(1) {
            if num - previous == 1 {
                previous = num;
            } else {
                result.push(if previous - start == 0 {
                    format!("{start}")
                } else {
                    format!("{start}->{previous}")
                });
                start = num;
                previous = num;
            }
        }
        result.push(if previous - start == 0 {
            format!("{start}")
        } else {
            format!("{start}->{previous}")
        });
        result
    }
}