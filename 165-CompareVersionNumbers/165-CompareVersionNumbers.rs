// Last updated: 23.09.2025, 10:32:48
use std::cmp::Ordering;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let (mut iter1, mut iter2) = (version1.split('.'), version2.split('.'));
        loop {
            match (iter1.next(), iter2.next()) {
                (Some(version1), Some(version2)) => {
                    let (version1, version2): (i32, i32) = (version1.parse().unwrap(), version2.parse().unwrap());
                    match version1.cmp(&version2) {
                        Ordering::Equal => continue,
                        Ordering::Greater => return 1,
                        Ordering::Less => return -1,
                    }
                }
                (Some(version), None) => if version.parse::<u32>().unwrap() > 0 { return 1; }
                (None, Some(version)) => if version.parse::<u32>().unwrap() > 0 { return -1; }
                _ => return 0,
            }
        }
    }
}