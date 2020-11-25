#![allow(unused)]
pub struct Solution {}

use std::cmp::min;
impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        if amount == 0 {
            return 0;
        }

        let mut dp = vec![amount + 1; amount as usize + 1];
        dp[0] = 0;

        for i in 1..dp.len() {
            for &coin in coins.iter() {
                let coin = coin as usize;
                if i >= coin {
                    dp[i] = min(dp[i - coin] + 1, dp[i]);
                }
            }
        }

        if dp[amount as usize] > amount {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_322() {
        assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
        assert_eq!(Solution::coin_change(vec![2], 3), -1);
        assert_eq!(Solution::coin_change(vec![1], 0), 0);
        assert_eq!(Solution::coin_change(vec![1], 1), 1);
        assert_eq!(Solution::coin_change(vec![1], 2), 2);
    }
}
