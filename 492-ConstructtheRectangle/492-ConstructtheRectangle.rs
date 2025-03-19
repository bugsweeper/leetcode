impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        for w in (1..=area.isqrt()).rev() {
            if area % w == 0 {
                return vec![area / w, w];
            }
        }
        Vec::new()
    }
}