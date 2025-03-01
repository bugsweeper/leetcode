// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut size = n;
        let mut base = 0;
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            if !self.isBadVersion(middle) {
                base = middle;
            }
            size -= half
        }
        base + 1
    }
}