#![allow(unused)]
pub struct Solution {}

use std::cmp::{max, min};
use std::i32::MAX;
impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; n + 1];

        for len in 2..=n {
            for start in 1..=n - len + 1 {
                let mut minres = MAX;
                for piv in start..start + len - 1 {
                    let res = piv as i32 + max(dp[start][piv - 1], dp[piv + 1][start + len - 1]);
                    minres = min(res, minres);
                }
                dp[start][start + len - 1] = minres;
            }
        }

        dp[1][n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_375() {
        assert_eq!(Solution::get_money_amount(10), 16);
        assert_eq!(Solution::get_money_amount(1), 0);
        assert_eq!(Solution::get_money_amount(2), 1);
    }
}
