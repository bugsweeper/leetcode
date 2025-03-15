impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut max = i32::MIN;
        let mut min = i32::MAX;
        for num in &nums {
            min = min.min(*num);
            max = max.max(*num);
        }
        if k == 1 {
            return min;
        }
        let mut base = min - 1;
        let mut size = max - min + 1;
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            let mut index = 0;
            let mut count = 0;
            while index < nums.len() {
                if nums[index] <= middle {
                    count += 1;
                    if count == k {
                        break;
                    }
                    index += 2;
                } else {
                    index += 1;
                }
            }
            if count >= k {
                max = middle;
            } else {
                base = middle;
            }
            size -= half;
        }
        max
    }
}