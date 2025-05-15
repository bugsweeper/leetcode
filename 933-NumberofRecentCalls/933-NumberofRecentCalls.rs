// Last updated: 15.05.2025, 14:18:52
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut trusts = vec![false; n + 1];
        let mut trusted = vec![0; n + 1];
        for relation in trust {
            let [a, b] = relation[..] else {
                unimplemented!()
            };
            trusts[a as usize] = true;
            trusted[b as usize] += 1; 
        }
        for (label, (trusts, trusted)) in trusts.into_iter().zip(trusted).enumerate().skip(1) {
            if !trusts && trusted == n - 1 {
                return label as i32;
            }
        }
        -1
    }
}