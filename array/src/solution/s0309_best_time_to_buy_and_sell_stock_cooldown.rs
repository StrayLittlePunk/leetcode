#![allow(unused)]
pub struct Solution {}

// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/

use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.is_empty() {
            return 0;
        }
        let n = prices.len();
        let mut f0 = -prices[0];
        let mut f1 = 0;
        let mut f2 = 0;
        for i in 1..n {
            let newf0 = max(f0, f2 - prices[i]);
            let newf1 = f0 + prices[i];
            let newf2 = max(f1, f2);
            f0 = newf0;
            f1 = newf1;
            f2 = newf2;
        }
        max(f1, f2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_309() {
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    }
}
