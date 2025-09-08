// Last updated: 08.09.2025, 16:18:38
impl Solution {
    pub fn best_hand(ranks: Vec<i32>, suits: Vec<char>) -> String {
        let first_suit = suits[0];
        if suits.into_iter().skip(1).all(|suit| suit == first_suit) {
            return "Flush".into();
        }
        let mut max_rank_count = 0;
        let mut rank_count = vec![0; 14];
        for rank in ranks {
            let rank_count = &mut rank_count[rank as usize];
            *rank_count += 1;
            if *rank_count == 3 {
                return "Three of a Kind".into();
            }
            max_rank_count = max_rank_count.max(*rank_count);
        }
        return if max_rank_count == 2 {
            "Pair"
        } else {
            "High Card"
        }.into()
    }
}