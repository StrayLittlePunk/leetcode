
#![allow(unused)]
pub struct Solution {}
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
impl Solution {
  pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() == 0{
      return 0;
    }
    use std::cmp::{max, min};
    let mut minprice: i32 = prices[0];
    let mut maxprofit: i32 = 0;
    for price in &prices{
      maxprofit = max(*price - minprice, maxprofit);
      minprice = min(*price, minprice);
    }
    maxprofit
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_121() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2]), 1);
    }
}
