
#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/


use std::cmp::max;
impl Solution {
  fn _quick_solve(prices: Vec<i32>) -> i32 {
    // we gain a profit as long as there is a price gap
    (1..prices.len()).map(|i| max(prices[i] - prices[i -1], 0)).sum()
  }
  pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    let n = prices.len();
    let k = k as usize;
    if k >= n / 2 {
      return Self::_quick_solve(prices);
    }

    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; k + 1];
    let mut ans = 0;
    for i in 1..k+1 {
      let mut pre_max = -prices[0];
      for j in 1..n{
        dp[i][j] = max(dp[i][j -1], pre_max + prices[j]);
        pre_max = max(pre_max, dp[i - 1][j- 1] - prices[j]);
        ans = max(ans, dp[i][j]);
      }
    }
    ans
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_123() {
        assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
        assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
    }
}
