// Last updated: 06.07.2025, 22:05:24
use std::collections::{HashMap, hash_map::Entry::Occupied};
use std::iter::once;

struct FindSumPairs {
    nums1: Vec<(i32, i32)>,
    nums2: Vec<i32>,
    num2count: HashMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FindSumPairs {

    fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        let mut nums1 = nums1;
        nums1.sort_unstable();
        let nums1 = nums1.iter().copied().chain(once(0)).fold((Vec::with_capacity(nums1.len()), nums1[0], 0), |(mut nums1, mut num, mut count), num1| {
            if num == num1 {
                count += 1;
            } else {
                nums1.push((num, count));
                num = num1;
                count = 1;
            }
            (nums1, num, count)
        }).0;
        let mut num2count = HashMap::with_capacity(nums2.len());
        for &num2 in &nums2 {
            num2count.entry(num2).and_modify(|count| *count += 1).or_insert(1);
        }
        Self {
            nums1,
            nums2,
            num2count,
        }
    }
    
    fn add(&mut self, index: i32, val: i32) {
        let num2 = &mut self.nums2[index as usize];
        if let Occupied(mut entry) = self.num2count.entry(*num2) {
            *entry.get_mut() -= 1;
            if *entry.get() == 0 {
                entry.remove_entry();
            }
        }
        *num2 += val;
        *self.num2count.entry(*num2).and_modify(|count| *count += 1).or_insert(1);
    }
    
    fn count(&self, tot: i32) -> i32 {
        let mut pair_count = 0;
        for &(num1, count1) in &self.nums1 {
            let num2 = tot - num1;
            if let Some(count2) = self.num2count.get(&num2) {
                pair_count += count1 * count2;
            }
        }
        pair_count
    }
}

/**
 * Your FindSumPairs object will be instantiated and called as such:
 * let obj = FindSumPairs::new(nums1, nums2);
 * obj.add(index, val);
 * let ret_2: i32 = obj.count(tot);
 */