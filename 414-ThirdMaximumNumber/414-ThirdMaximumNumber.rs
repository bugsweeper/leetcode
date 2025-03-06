impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut maximums = Vec::with_capacity(4);
        for num in nums {
            let insert_position = maximums.partition_point(|max| *max > num);
            if insert_position < 3 {
                if insert_position < maximums.len() {
                    if maximums[insert_position] != num {
                        maximums.insert(insert_position, num);
                    }
                } else {
                    maximums.push(num);
                }
                maximums.truncate(3);
            }
        }
        if maximums.len() < 3 {
            maximums[0]
        } else {
            maximums[2]
        }
    }
}