#![allow(unused)]
pub struct Solution {}

use std::cmp::{max, min};
use std::i32::MAX;
impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let m = dungeon.len();
        let n = dungeon[0].len();
        let mut dp = vec![vec![MAX; n]; m];
        dp[m - 1][n - 1] = 1;
        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i > 0 {
                    dp[i - 1][j] = min(max(dp[i][j] - dungeon[i][j], 1), dp[i - 1][j]);
                }
                if j > 0 {
                    dp[i][j - 1] = min(max(dp[i][j] - dungeon[i][j], 1), dp[i][j - 1]);
                }
            }
        }
        max(dp[0][0] - dungeon[0][0], 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_174() {
        assert_eq!(
            Solution::calculate_minimum_hp(vec![
                vec![-2, -3, 3],
                vec![-5, -10, 1],
                vec![10, 30, -5]
            ]),
            7
        );
    }
}
