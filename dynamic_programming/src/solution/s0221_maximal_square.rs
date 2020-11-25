#![allow(unused)]
pub struct Solution {}

use std::cmp::{max, min};
use std::i32::MAX;
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.len() == 0 || matrix[0].len() == 0 {
            return 0;
        }
        let (rows, cols) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; cols + 1]; rows + 1];

        let mut maxsqlen = 0;
        for i in 1..=rows {
            for j in 1..=cols {
                if matrix[i - 1][j - 1] == '1' {
                    dp[i][j] = min(min(dp[i][j - 1], dp[i - 1][j]), dp[i - 1][j - 1]) + 1;
                    maxsqlen = max(maxsqlen, dp[i][j]);
                }
            }
        }

        maxsqlen * maxsqlen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_221() {
        assert_eq!(
            Solution::maximal_square(vec![
                vec!['1', '0', '1', '0', '0'],
                vec!['1', '0', '1', '1', '1'],
                vec!['1', '1', '1', '1', '1'],
                vec!['1', '0', '0', '1', '0']
            ]),
            4
        );
    }
}
