impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min1 = Some(unsafe{*nums.get_unchecked(0)});
        let mut min2 = None;
        let mut medium = None;
        for &num in &nums[1..] {
            match (min1, medium, min2) {
                (_, _, Some(value)) if value == num => {},
                (Some(min1_value), None, _) => 
                    if num < min1_value {
                        min1 = Some(num);
                    } else {
                        if num != min1_value {
                            medium = Some(num);
                        }
                    },
                (_, _, Some(min2_value)) if num < min2_value => min2 = Some(num),
                (Some(min1_value), _, None) if num < min1_value => min2 = Some(num),
                (_, Some(medium_value), _) => {
                    if num > medium_value {
                        return true;
                    } else {
                        if min2.is_some() {
                            medium = Some(num);
                            min1 = min2;
                            min2 = None;
                        } else {
                            if min1 != Some(num) {
                                medium = Some(num);
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        false
    }
}