// Last updated: 16.05.2025, 16:53:38
fn num_squares(n: usize, cache: &mut [usize]) -> usize {
    if cache[n] != 0 {
        return cache[n];
    }
    let max_root = n.isqrt();
    let mut min_num = n;// worst case if it contains only ones
    for root in (1..=max_root).rev() {
        let square = root * root;
        if min_num <= n / square {
            break;
        }
        let num = num_squares(n - square, cache) + 1;
        min_num = min_num.min(num);
    }
    cache[n] = min_num;
    min_num
}

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut cache = vec![0; n + 1];
        cache[1] = 1;
        num_squares(n, &mut cache) as i32

    }
}