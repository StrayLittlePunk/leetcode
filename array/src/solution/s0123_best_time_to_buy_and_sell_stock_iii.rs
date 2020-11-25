
#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/

impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
      return 0;
    }
    use std::cmp::max;
    let max_trans = 2;
    let mut cache = vec![0; prices.len()];
    for trans in 0..max_trans {
      let mut best_buy_in = cache[0] - prices[0];
      for i in 1..prices.len() {
        let temp = cache[i];
        cache[i] = max(cache[i -1 ], best_buy_in + prices[i]);
        best_buy_in = max(best_buy_in, temp - prices[i]);
      }
    }
    return *cache.last().unwrap();
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
