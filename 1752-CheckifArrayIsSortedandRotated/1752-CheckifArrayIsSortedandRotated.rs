impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let mut previous = *nums.first().unwrap();
        let mut gap_found = false;
        for &num in nums.iter().skip(1) {
            if num < previous {
                if gap_found {
                    return false;
                }
                let last = *nums.last().unwrap();
                if num > last || last > *nums.first().unwrap() {
                    return false;
                }
                gap_found = true;
            }
            previous = num;
        }
        true
    }
}