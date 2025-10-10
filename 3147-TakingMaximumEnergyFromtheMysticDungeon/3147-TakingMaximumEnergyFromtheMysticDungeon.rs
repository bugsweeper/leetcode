// Last updated: 10.10.2025, 06:12:40
impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let mut dp = Vec::with_capacity(energy.len() + 1);
        dp.push(0);
        let k = k as usize;
        let mut max = i32::MIN;
        for energy in energy.into_iter().rev() {
            let cur_energy = dp[dp.len().saturating_sub(k)] + energy;
            max = max.max(cur_energy);
            dp.push(cur_energy);
        }
        max
    }
}