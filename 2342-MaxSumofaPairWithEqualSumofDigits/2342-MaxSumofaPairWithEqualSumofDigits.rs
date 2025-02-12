impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut pairs = vec![(0, 0); 82];
        for num in nums {
            let mut digit_sum = 0;
            {
                let mut num = num;
                while num > 0 {
                    digit_sum += num % 10;
                    num /= 10;
                }
            }
            let (max1, max2) = unsafe { pairs.get_unchecked_mut(digit_sum as usize) };
            if num >= *max1 {
                *max2 = *max1;
                *max1 = num;
            } else if num >= *max2 {
                *max2 = num;
            }
        }
        pairs
            .into_iter()
            .map(|(max1, max2)| {
                if max1 > 0 && max2 > 0 {
                    max1 + max2
                } else {
                    -1
                }
            })
            .max()
            .unwrap_or(-1)
    }
}