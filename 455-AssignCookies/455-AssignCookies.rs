impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        let mut g = g;
        g.sort_unstable();
        let mut s = s;
        s.sort_unstable();
        let mut s_iter = s.into_iter();
        let mut result = 0;
        'g: for greed_factor in g {
            for size in s_iter.by_ref() {
                if size >= greed_factor {
                    result += 1;
                    continue 'g;
                }
            }
            break;
        }
        result
    }
}