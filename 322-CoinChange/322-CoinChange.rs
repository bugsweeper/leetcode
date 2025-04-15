// Last updated: 15.04.2025, 22:52:57
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut count = vec![-1; amount + 1];
        count[0] = 0;
        for index in 1..=amount {
            let mut min_coins = i32::MAX;
            for &coin in &coins {
                let coin = coin as usize;
                if coin <= index {
                    let prev_count = count.get(index - coin).unwrap();
                    if *prev_count != -1 {
                        min_coins = min_coins.min(*prev_count + 1);
                    }
                }
            }
            if min_coins != i32::MAX {
                count[index] = min_coins;
            }
        }
        count[amount]
    }
}