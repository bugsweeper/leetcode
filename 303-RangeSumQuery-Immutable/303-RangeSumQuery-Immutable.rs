struct NumArray {
    prefix_sum: Vec<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut nums = nums;
        let mut sum = 0;
        for num in &mut nums {
            *num += sum;
            sum = *num;
        }
        Self { prefix_sum: nums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        if left == 0 {
            self.prefix_sum[right as usize]
        } else {
            self.prefix_sum[right as usize] - self.prefix_sum[left as usize - 1]
        }
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */