// Last updated: 27.05.2025, 23:15:20
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let k = k as usize;
        // odd cells keep best buy results, even cells keep best sell results
        let mut transaction_profit = vec![0; 2 * k];
        for sell_transaction in transaction_profit.iter_mut().step_by(2) {
            *sell_transaction = i32::MIN;
        }
        for price in prices {
            let first = transaction_profit.get_mut(0).unwrap();
            *first = (*first).max(-price);
            let mut prev_value = *first;
            for (index, transaction) in transaction_profit.iter_mut().enumerate().skip(1) {
                if index & 1 == 0 {
                    // buy
                    *transaction = (prev_value - price).max(*transaction);
                } else {
                    // sell
                    *transaction = (prev_value + price).max(*transaction);
                }
                prev_value = *transaction;
            }
        }
        *transaction_profit.last().unwrap()
    }
}