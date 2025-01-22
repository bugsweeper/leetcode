use std::cmp::Ordering;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut max_length = 0;
        let mut start2length = std::collections::BTreeMap::new();
        for num in nums {
            let mut range = start2length.range(num - max_length..=num + 1);
            let (insert_num, insert_len, remove) =
                if let Some((&start, &length)) = range.next_back() {
                    if start > num {
                        if let Some((&previous_start, &previous_length)) = range.next_back() {
                            if previous_start + previous_length == num {
                                (previous_start, previous_length + 1 + length, Some(start))
                            } else {
                                (num, 1 + length, Some(start))
                            }
                        } else {
                            (num, 1 + length, Some(start))
                        }
                    } else {
                        let end: i32 = start + length;
                        match end.cmp(&num) {
                            Ordering::Equal => (start, length + 1, None),
                            Ordering::Greater => continue,
                            Ordering::Less => (num, 1, None),
                        }
                    }
                } else {
                    (num, 1, None)
                };
            max_length = max_length.max(insert_len);
            start2length.insert(insert_num, insert_len);
            if let Some(remove) = remove {
                start2length.remove(&remove);
            }
        }
        max_length
    }
}