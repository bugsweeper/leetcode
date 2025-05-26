// Last updated: 26.05.2025, 21:45:40
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // Keeps best transactions for n right days
        let mut right_transaction = Vec::with_capacity(prices.len());
        right_transaction.push(0);
        let mut max_sell = *prices.last().unwrap();
        let mut max_transaction = 0;
        for &price in prices.iter().rev().skip(1) {
            max_transaction = max_transaction.max(max_sell - price);
            right_transaction.push(max_transaction);
            max_sell = max_sell.max(price);
        }
        let mut max_sum = max_transaction;
        // Now max_transaction is current best left transaction
        max_transaction = 0;
        let mut min_buy = *prices.first().unwrap();
        let _ = right_transaction.pop();
        for price in prices.into_iter().skip(1) {
            max_transaction = max_transaction.max(price - min_buy);
            max_sum = max_sum.max(max_transaction + right_transaction.pop().unwrap());
            min_buy = min_buy.min(price);
        }
        max_sum
    }
}