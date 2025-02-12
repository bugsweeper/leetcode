impl Solution {
    pub fn maximum_sum(nums: Vec<i32>) -> i32 {
        let mut digit_sum2max = [0; 82];
        let mut max_sum = -1;
        for num in nums {
            let mut digit_sum = 0;
            {
                let mut num = num;
                while num > 0 {
                    digit_sum += num % 10;
                    num /= 10;
                }
            }
            let prev_max = *unsafe { digit_sum2max.get_unchecked(digit_sum as usize) };
            if prev_max > 0 {
                max_sum = max_sum.max(prev_max + num);
            }
            if num > prev_max {
                *unsafe { digit_sum2max.get_unchecked_mut(digit_sum as usize) } = num;
            }
        }
        max_sum
    }
}
