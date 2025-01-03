impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut first = i32::MAX;
        let mut second = i32::MAX;

        for n in nums {
            if first >= n {
                first = n;
            } else if second >= n {
                second = n;
            } else {
                return true;
            }
        }

        false
    }
}
