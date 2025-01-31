impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let max_profit = 0;
        let min_value = prices[0];
        let (_, max_profit) =
            prices[1..]
                .iter()
                .fold((min_value, max_profit), |(min_value, max_profit), price| {
                    if *price < min_value {
                        (*price, max_profit)
                    } else {
                        (min_value, max_profit.max(*price - min_value))
                    }
                });
        max_profit
    }
}