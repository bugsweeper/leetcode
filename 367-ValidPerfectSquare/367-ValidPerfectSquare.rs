impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut base = 1;
        let mut size = num;
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            if middle <= num / middle {
                base = middle;
            }
            size -= half;
        }
        base * base == num
    }
}